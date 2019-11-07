use crate::{
    event::Event,
    topology::config::{DataType, GlobalOptions, SourceConfig},
};
use futures::{future, sync::mpsc, Future, Sink};
use serde::{Deserialize, Serialize};
use rumqtt::{MqttClient, MqttOptions, QoS, Notification};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MqttConfig {
    pub address: String,
    pub port: u16,
    pub topics: Vec<String>,
}

impl MqttConfig {
    pub fn new(address: String, port: u16, topics: Vec<String>) -> Self {
        Self {
            address,
            port,
            topics,
        }
    }
}

#[typetag::serde(name = "mqtt")]
impl SourceConfig for MqttConfig {
    fn build(
        &self,
        _name: &str,
        _globals: &GlobalOptions,
        out: mpsc::Sender<Event>,
    ) -> crate::Result<super::Source> {
        Ok(mqtt(self.address.clone(), self.port, self.topics.clone(), out))
    }

    fn output_type(&self) -> DataType {
        DataType::Log
    }
}

pub fn generate_event_from_notification(notification: Notification) -> Option<Event> {
    match notification {
        rumqtt::Notification::Publish(publish) => {
            let payload = std::str::from_utf8(&(*publish.payload)[..]);
            match payload {
                Ok(data) => {
                    let event = Event::from(data);
                    Some(event)
                }
                _ => {None}
            }

        }
        _ => {None}
    }
}

pub fn mqtt(address: String, port: u16, topics: Vec<String>, out: mpsc::Sender<Event>) -> super::Source {
    let out = out.sink_map_err(|e| error!("error sending event: {:?}", e));

    Box::new(
        future::lazy(move || {
            let mqtt_options = MqttOptions::new("vector-source", address, port);
            let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
            for topic in topics {
                mqtt_client.subscribe(topic, QoS::AtLeastOnce).unwrap();
            }

            for notification in notifications {
                let event = generate_event_from_notification(notification);

                if let Some(data) = event {
                    match out.clone().send(data).wait() {
                        Ok(_) => {}
                        Err(()) => error!(message = "Could not send journald log"),
                    }
                }
            }

            Ok(mqtt_client)
        }).and_then(move | mut mqtt_client| {
            mqtt_client.shutdown().unwrap();
            future::empty()
        })
    )
}


#[cfg(test)]
mod test {

    use crate::event;
    use rumqtt::{MqttClient, MqttOptions, QoS};
    use super::MqttConfig;
    use crate::sources::mqtt::{mqtt, generate_event_from_notification};
    use futures::sync::mpsc;
    use futures::Stream;
    use std::sync::Arc;
    use rdkafka::message::ToBytes;
    use rumqtt::PacketIdentifier;
    use rumqtt::Publish;
    use rumqtt::QoS::AtLeastOnce;
    use stream_cancel::StreamExt;
    use crate::test_util::{block_on, collect_n};
    use serde_json::value::Index;
    use std::time::Duration;
    use std::thread;

    #[test]
    fn test_generate_event(){

        let msg = rumqtt::client::Notification::Publish(Publish {
            dup: false,
            qos: AtLeastOnce,
            retain: false,
            pkid: Some(PacketIdentifier(0)),
            topic_name: "test".to_owned(),
            payload: Arc::new("test_msg".to_bytes().to_vec()),
        });

        let event = generate_event_from_notification(msg);
        assert_eq!(event.unwrap().as_log()[&event::MESSAGE], "test_msg".into())
    }

    fn init_mqtt(sender: mpsc::Sender<event::Event>, address: String, port: u16,
                 topics: Vec<String>) -> tokio::runtime::Runtime {

        let source = mqtt(address, port,
                           topics, sender);
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        rt.spawn(source);

        // Wait for mqtt to start listening
        thread::sleep(Duration::from_millis(300));

        rt
    }

    #[test]
    fn test_messages(){

        let address = String::from("broker.hivemq.com");
        let port = 1883;
        let topic = String::from("test_topic_1");
        let topics = vec![topic.clone()];

        let (tx, rx) =  mpsc::channel::<event::Event>(10);


        let mut rt = init_mqtt(tx, address.clone(), port, topics);


        // Send tests
        let mqtt_options = MqttOptions::new("vector-source", &address, port);
        let (mut mqtt_client, notifications) =
            MqttClient::start(mqtt_options).unwrap();

        mqtt_client.publish(&topic, QoS::AtLeastOnce, false, "test").unwrap();
        mqtt_client.publish(&topic, QoS::AtLeastOnce, false, "test").unwrap();


        let events = rt.block_on(collect_n(rx, 2)).ok().unwrap();

        assert_eq!(events[0].as_log()[&event::MESSAGE], "test".into());
        assert_eq!(events[1].as_log()[&event::MESSAGE], "test2".into());
    }
}
