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
            topics: topics,
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
        Ok(mqtt(self.address, out))
    }

    fn output_type(&self) -> DataType {
        DataType::Log
    }
}

pub fn mqtt(address: SocketAddr, out: mpsc::Sender<Event>) -> super::Source {
    let out = out.sink_map_err(|e| error!("error sending event: {:?}", e));

    Box::new(
        future::lazy(move || {
            let mqtt_options = MqttOptions::new("test-pubsub1", "broker.hivemq.com", 1883);
            let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
            mqtt_client.subscribe("hello/world", QoS::AtLeastOnce).unwrap();


            for notification in notifications {
                match notification {
                    rumqtt::Notification::Publish(publish) => {
                        Event::from(publish.payload.into());
                    }
                    _ => {}
                }
            }

            future::empty()
        })
    )
}