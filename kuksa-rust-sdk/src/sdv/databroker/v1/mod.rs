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

use crate::kuksa::common::types;
use std::collections::HashMap;

use crate::kuksa::common::{Client, ClientError, SDVClientTraitV1};
use crate::proto::sdv::databroker::v1 as proto;
use http::Uri;
use tonic::async_trait;

pub struct SDVClient {
    pub basic_client: Client,
}

impl SDVClient {
    pub fn new(uri: Uri) -> Self {
        SDVClient {
            basic_client: Client::new(uri),
        }
    }

    pub fn from_host(host: &'static str) -> Self {
        let uri = Uri::from_static(host);
        Self::new(uri)
    }
}

#[async_trait]
impl SDVClientTraitV1 for SDVClient {
    type SensorUpdateType = types::SensorUpdateSDVTypeV1;
    type UpdateActuationType = types::UpdateActuationSDVTypeV1;
    type PathType = types::PathSDVTypeV1;
    type SubscribeType = types::SubscribeSDVTypeV1;
    type PublishResponseType = types::PublishResponseSDVTypeV1;
    type GetResponseType = types::GetResponseSDVTypeV1;
    type SubscribeResponseType = types::SubscribeResponseSDVTypeV1;
    type ProvideResponseType = types::ProvideResponseSDVTypeV1;
    type ActuateResponseType = types::ActuateResponseSDVTypeV1;
    type MetadataResponseType = types::MetadataResponseSDVTypeV1;

    async fn update_datapoints(
        &mut self,
        datapoints: Self::SensorUpdateType,
    ) -> Result<Self::PublishResponseType, ClientError> {
        let metadata = self
            .get_metadata(datapoints.keys().cloned().collect())
            .await
            .unwrap();
        let id_datapoints: HashMap<i32, proto::Datapoint> = metadata
            .into_iter()
            .map(|meta| meta.id)
            .zip(datapoints.into_values())
            .collect();

        let mut client = proto::collector_client::CollectorClient::with_interceptor(
            self.basic_client.get_channel().await?.clone(),
            self.basic_client.get_auth_interceptor(),
        );

        let request = tonic::Request::new(proto::UpdateDatapointsRequest {
            datapoints: id_datapoints,
        });
        match client.update_datapoints(request).await {
            Ok(response) => Ok(response.into_inner()),
            Err(err) => Err(ClientError::Status(err)),
        }
    }

    async fn get_datapoints(
        &mut self,
        paths: Self::PathType,
    ) -> Result<Self::GetResponseType, ClientError> {
        let mut client = proto::broker_client::BrokerClient::with_interceptor(
            self.basic_client.get_channel().await?.clone(),
            self.basic_client.get_auth_interceptor(),
        );
        let args = tonic::Request::new(proto::GetDatapointsRequest { datapoints: paths });
        match client.get_datapoints(args).await {
            Ok(response) => {
                let message = response.into_inner();
                Ok(message.datapoints)
            }
            Err(err) => Err(ClientError::Status(err)),
        }
    }

    async fn subscribe(
        &mut self,
        paths: Self::SubscribeType,
    ) -> Result<Self::SubscribeResponseType, ClientError> {
        let mut client = proto::broker_client::BrokerClient::with_interceptor(
            self.basic_client.get_channel().await?.clone(),
            self.basic_client.get_auth_interceptor(),
        );
        let args = tonic::Request::new(proto::SubscribeRequest { query: paths });

        match client.subscribe(args).await {
            Ok(response) => Ok(response.into_inner()),
            Err(err) => Err(ClientError::Status(err)),
        }
    }

    async fn set_datapoints(
        &mut self,
        datapoints: Self::UpdateActuationType,
    ) -> Result<Self::ActuateResponseType, ClientError> {
        let args = tonic::Request::new(proto::SetDatapointsRequest { datapoints });
        let mut client = proto::broker_client::BrokerClient::with_interceptor(
            self.basic_client.get_channel().await?.clone(),
            self.basic_client.get_auth_interceptor(),
        );
        match client.set_datapoints(args).await {
            Ok(response) => Ok(response.into_inner()),
            Err(err) => Err(ClientError::Status(err)),
        }
    }

    async fn get_metadata(
        &mut self,
        paths: Self::PathType,
    ) -> Result<Self::MetadataResponseType, ClientError> {
        let mut client = proto::broker_client::BrokerClient::with_interceptor(
            self.basic_client.get_channel().await?.clone(),
            self.basic_client.get_auth_interceptor(),
        );
        // Empty vec == all property metadata
        let args = tonic::Request::new(proto::GetMetadataRequest { names: paths });
        match client.get_metadata(args).await {
            Ok(response) => {
                let message = response.into_inner();
                Ok(message.list)
            }
            Err(err) => Err(ClientError::Status(err)),
        }
    }
}
