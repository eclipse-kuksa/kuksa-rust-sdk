/*
 * *******************************************************************************
 *  Copyright (c) 2025 Contributors to the Eclipse Foundation
 *
 *  See the NOTICE file(s) distributed with this work for additional
 *  information regarding copyright ownership.
 *
 *  This program and the accompanying materials are made available under the
 *  terms of the Apache License 2.0 which is available at
 *  http://www.apache.org/licenses/LICENSE-2.0
 *
 *  SPDX-License-Identifier: Apache-2.0
 * ******************************************************************************
 */
use kuksa_rust_sdk::{sdv_proto, v1_proto, v2_proto};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let host = if cfg!(target_os = "macos") {
        "localhost:55556"
    } else {
        "localhost:55555"
    };

    execute_v2_calls(host).await;
    execute_v1_calls(host).await;
    execute_sdv_calls(host).await;
}

async fn execute_v2_calls(host: &'static str) {
    let mut v2_client = kuksa_rust_sdk::v2::KuksaClientV2::from_host(host);

    match v2_client.subscribe(vec!["Vehicle.Speed"], None).await {
        Ok(_) => {
            println!("Successfully subscribed to {:?}!", "Vehicle.Speed");
        }
        Err(err) => {
            println!("Failed to subscribe to {:?}: {:?}", "Vehicle.Speed", err);
        }
    }

    match v2_client
        .publish_value(
            "Vehicle.Speed",
            v2_proto::Value {
                typed_value: Some(v2_proto::value::TypedValue::Float(30.0)),
            },
        )
        .await
    {
        Ok(_) => {
            println!(
                "Value published successful for signal {:?}",
                "Vehicle.Speed"
            );
        }
        Err(err) => {
            println!(
                "Publishing value for signal {:?} failed: {:?}",
                "Vehicle.Speed", err
            );
        }
    }

    match v2_client.get_value("Vehicle.Speed").await {
        Ok(response) => {
            println!("Got value for Vehicle.Speed: {:?}", response);
        }
        Err(err) => {
            println!(
                "Getting value for signal {:?} failed: {:?}",
                "Vehicle.Speed", err
            );
        }
    }
}

async fn execute_v1_calls(host: &'static str) {
    let mut v1_client = kuksa_rust_sdk::v1::KuksaClient::from_host(host);

    match v1_client.subscribe(vec!["Vehicle.Speed"]).await {
        Ok(_) => {
            println!("Successfully subscribed to {:?}!", "Vehicle.Speed");
        }
        Err(err) => {
            println!(
                "Failed to subscribe {:?} failed: {:?}",
                "Vehicle.Speed", err
            );
        }
    }

    let mut datapoints = HashMap::with_capacity(1);
    datapoints.insert(
        "Vehicle.Speed".to_owned(),
        v1_proto::Datapoint {
            timestamp: None,
            value: Some(v1_proto::datapoint::Value::Float(40.0)),
        },
    );

    match v1_client.set_current_values(datapoints).await {
        Ok(_) => {
            println!("Successfully set datapoints")
        }
        Err(err) => {
            println!("Failed to set datapoints: {:?}", err)
        }
    }

    match v1_client
        .get_current_values(vec!["Vehicle.Speed".to_owned()])
        .await
    {
        Ok(response) => {
            println!("Got value for Vehicle.Speed: {:?}", response);
        }
        Err(err) => {
            println!("Couldn't get value for Vehicle.Speed: {:?}", err)
        }
    }
}

async fn execute_sdv_calls(host: &'static str) {
    let mut sdv_client = kuksa_rust_sdk::sdv::SDVClient::from_host(host);

    match sdv_client.subscribe("Vehicle.Speed".to_owned()).await {
        Ok(_) => {
            println!("Successfully subscribed to {:?}!", "Vehicle.Speed");
        }
        Err(err) => {
            println!("Failed to subscribe to {:?}: {:?}", "Vehicle.Speed", err);
        }
    }

    let mut datapoints = HashMap::with_capacity(1);
    datapoints.insert(
        "Vehicle.Speed".to_owned(),
        sdv_proto::Datapoint {
            timestamp: None,
            value: Some(sdv_proto::datapoint::Value::FloatValue(50.0)),
        },
    );
    match sdv_client.set_datapoints(datapoints).await {
        Ok(_) => {
            println!("Successfully set datapoints")
        }
        Err(err) => {
            println!("Failed to set datapoints: {:?}", err)
        }
    }

    match sdv_client.get_datapoints(vec!["Vehiclce.Speed"]).await {
        Ok(response) => {
            println!("Got value for Vehicle.Speed: {:?}", response);
        }
        Err(err) => {
            println!("Failed to get value for Vehicle.Speed: {:?}", err)
        }
    }
}
