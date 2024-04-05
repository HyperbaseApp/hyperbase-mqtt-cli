use std::path::PathBuf;

use clap::Parser;
use hyperbase_mqtt_lib::{payload::Payload, version::MqttVersion};
use rumqttc::{v5, AsyncClient, Event, MqttOptions, Packet, QoS};
use tokio::fs;
use uuid::Uuid;

use crate::config::Config;

mod config;

#[derive(Parser)]
struct Args {
    path: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let file_content = match fs::read_to_string(&args.path).await {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Err: Could not read config file at {:?}", args.path);
            eprintln!("Err Msg: {err}");
            return;
        }
    };
    let config = serde_json::from_str::<Config>(&file_content).unwrap();

    let mut mqtt_version = MqttVersion::V5;
    if let Some(v) = config.broker.mqtt_version {
        if v.to_lowercase() == "v3" {
            mqtt_version = MqttVersion::V3
        }
    }

    println!("Using MQTT Version {}", mqtt_version.to_str());

    match mqtt_version {
        MqttVersion::V3 => {
            let mut mqtt_options_v3 =
                MqttOptions::new(Uuid::now_v7(), &config.broker.host, config.broker.port);
            mqtt_options_v3.set_credentials(&config.broker.token_id, &config.broker.token);

            let (client, mut eventloop) = AsyncClient::new(mqtt_options_v3, 10);

            client
                .subscribe(&config.broker.topic, QoS::AtMostOnce)
                .await
                .unwrap();

            loop {
                match eventloop.poll().await {
                    Ok(event) => {
                        if let Event::Incoming(packet) = event {
                            if let Packet::Publish(publish) = packet {
                                match serde_json::from_slice::<Payload>(&publish.payload) {
                                    Ok(payload) => {
                                        if payload.collection_id == config.payload.collection_id {
                                            println!("Incoming payload packet");
                                            println!("{payload:#?}")
                                        }
                                    }
                                    Err(err) => {
                                        eprintln!("Err: Failed to deserialize packet to hyperbase mqtt payload");
                                        eprintln!("Err Msg: {err}");
                                    }
                                }
                            } else {
                                println!("Incoming packet: {packet:?}");
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!(
                            "Err: Failed to polling mqtt broker using token_id '{}' and token '{}'",
                            config.broker.token_id, config.broker.token
                        );
                        eprintln!("Err Msg: {err}");
                        return;
                    }
                }
            }
        }
        MqttVersion::V5 => {
            let mut mqtt_options_v5 =
                v5::MqttOptions::new(Uuid::now_v7(), &config.broker.host, config.broker.port);
            mqtt_options_v5.set_credentials(&config.broker.token_id, &config.broker.token);

            let (client, mut eventloop) = v5::AsyncClient::new(mqtt_options_v5, 10);

            client
                .subscribe(&config.broker.topic, v5::mqttbytes::QoS::AtMostOnce)
                .await
                .unwrap();

            loop {
                match eventloop.poll().await {
                    Ok(event) => {
                        if let v5::Event::Incoming(packet) = event {
                            if let v5::mqttbytes::v5::Packet::Publish(publish) = packet {
                                match serde_json::from_slice::<Payload>(&publish.payload) {
                                    Ok(payload) => {
                                        println!("Incoming payload packet");
                                        println!("{payload:#?}")
                                    }
                                    Err(err) => {
                                        eprintln!("Err: Failed to deserialize packet to hyperbase mqtt payload");
                                        eprintln!("Err Msg: {err}");
                                    }
                                }
                            } else {
                                println!("Incoming packet: {packet:?}");
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!(
                            "Err: Failed to polling mqtt broker using token_id '{}' and token '{}'",
                            config.broker.token_id, config.broker.token
                        );
                        eprintln!("Err Msg: {err}");
                        return;
                    }
                }
            }
        }
    }
}
