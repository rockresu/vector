use crate::{
    event::Event,
    topology::config::{DataType, GlobalOptions, SourceConfig},
};
use futures::{future, sync::mpsc, Future, Sink};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr};
use rumqtt::{MqttClient, MqttOptions, QoS};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MqttConfig {
    pub address: SocketAddr,
    pub topics: Vec<String>,
}

impl MqttConfig {
    pub fn new(address: SocketAddr, topics: Vec<String>) -> Self {
        Self {
            address,
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
        Ok(mqtt(self.address, self.topics.clone(), out))
    }

    fn output_type(&self) -> DataType {
        DataType::Log
    }
}

pub fn mqtt(address: SocketAddr, topics: Vec<String>, out: mpsc::Sender<Event>) -> super::Source {
    let out = out.sink_map_err(|e| error!("error sending event: {:?}", e));

    Box::new(
        future::lazy(move || {
            let mqtt_options = MqttOptions::new("test-pubsub1", address.ip().to_string(), address.port());
            let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
            for topic in topics {
                mqtt_client.subscribe(topic, QoS::AtLeastOnce).unwrap();
            }

            Ok(notifications)
        }).and_then(move |notifications| {

            for notification in notifications {
                let event = match notification {
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
                };

                if let Some(data) = event {
                    match out.clone().send(data).wait() {
                        Ok(_) => {}
                        Err(()) => error!(message = "Could not send journald log"),
                    }
                }
            }
            future::empty()
//            Ok("Finished");
        })
    )
}