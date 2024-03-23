use std::{path::PathBuf, time::Duration};

use clap::Parser;
use hyperbase_mqtt_lib::version::MqttVersion;
use rumqttc::{mqttbytes, v5, AsyncClient, Event, MqttOptions, Outgoing};
use tokio::{fs, task};
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
    if let Some(v) = config.mqtt_version {
        if v.to_lowercase() == "v3" {
            mqtt_version = MqttVersion::V3
        }
    }

    println!("Using MQTT Version {}", mqtt_version.to_str());

    let mut count = 1;
    if let Some(c) = config.count {
        if c > 1 {
            count = c;
        }
    }
    let mut delay = Duration::ZERO;
    if let Some(d) = config.delay {
        if d > 0 {
            delay = Duration::from_secs(d)
        }
    }

    let payload_string = serde_json::to_string(&config.payload).unwrap();

    match mqtt_version {
        MqttVersion::V3 => {
            let mqtt_options_v3 = MqttOptions::new(Uuid::now_v7(), &config.host, config.port);
            let (client, mut eventloop) = AsyncClient::new(mqtt_options_v3, 10);

            task::spawn(async move {
                for _ in 0..count {
                    if let Err(err) = client
                        .publish(
                            &config.topic,
                            mqttbytes::QoS::AtMostOnce,
                            false,
                            payload_string.clone(),
                        )
                        .await
                    {
                        eprintln!(
                            "Err: Failed to publish to mqtt broker v3 {}:{} with payload {}",
                            config.host, config.port, payload_string
                        );
                        eprintln!("Err Msg: {err}");
                        return;
                    }
                    tokio::time::sleep(delay).await;
                }
            });

            let mut published_count = 0;
            while let Ok(event) = eventloop.poll().await {
                match event {
                    Event::Incoming(packet) => {
                        println!("Incoming = {packet:?}")
                    }
                    Event::Outgoing(outgoing) => {
                        if let Outgoing::Publish(_) = outgoing {
                            published_count += 1;
                            println!("Publish {published_count}");
                            if published_count == count {
                                println!("Successfully published all data");
                                break;
                            }
                        } else {
                            println!("Outgoing = {outgoing:?}");
                        }
                    }
                }
            }
        }
        MqttVersion::V5 => {
            let mqtt_options_v5 = v5::MqttOptions::new(Uuid::now_v7(), &config.host, config.port);
            let (client, mut eventloop) = v5::AsyncClient::new(mqtt_options_v5, 10);

            task::spawn(async move {
                for _ in 0..count {
                    if let Err(err) = client
                        .publish(
                            &config.topic,
                            v5::mqttbytes::QoS::AtMostOnce,
                            false,
                            payload_string.clone(),
                        )
                        .await
                    {
                        eprintln!(
                            "Err: Failed to publish to mqtt broker v5 {}:{} with payload {}",
                            config.host, config.port, payload_string
                        );
                        eprintln!("Err Msg: {err}");
                        return;
                    }
                    tokio::time::sleep(delay).await;
                }
            });

            let mut published_count = 0;
            while let Ok(event) = eventloop.poll().await {
                match event {
                    v5::Event::Incoming(packet) => {
                        println!("Incoming = {packet:?}")
                    }
                    v5::Event::Outgoing(outgoing) => {
                        if let Outgoing::Publish(_) = outgoing {
                            published_count += 1;
                            println!("Publish {published_count}");
                            if published_count == count {
                                println!("Successfully published all data");
                                break;
                            }
                        } else {
                            println!("Outgoing = {outgoing:?}");
                        }
                    }
                }
            }
        }
    }
}
