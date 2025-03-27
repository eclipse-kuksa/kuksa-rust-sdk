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

use crate::kuksa::common;
use crate::kuksa::common::conversion::{ConvertToSDV, ConvertToV1};
use crate::kuksa::common::types;
use crate::kuksa::common::ClientTraitV1;
use http::Uri;
use tonic::async_trait;

pub use databroker_proto::kuksa::val::{self as proto, v1::DataEntry};

pub use crate::kuksa::common::{Client, ClientError};

#[derive(Debug)]
pub struct KuksaClient {
    pub basic_client: Client,
}

impl KuksaClient {
    pub fn new(uri: Uri) -> Self {
        KuksaClient {
            basic_client: Client::new(uri),
        }
    }

    pub fn from_host(host: &'static str) -> Self {
        let uri = Uri::from_static(host);
        Self::new(uri)
    }

    async fn set(&mut self, entry: DataEntry, _fields: Vec<i32>) -> Result<(), ClientError> {
        let mut client = proto::v1::val_client::ValClient::with_interceptor(
            self.basic_client.get_channel().await?.clone(),
            self.basic_client.get_auth_interceptor(),
        );
        let set_request = proto::v1::SetRequest {
            updates: vec![proto::v1::EntryUpdate {
                entry: Some(entry),
                fields: _fields,
            }],
        };
        match client.set(set_request).await {
            Ok(response) => {
                let message = response.into_inner();
                let mut errors: Vec<proto::v1::Error> = Vec::new();
                if let Some(err) = message.error {
                    errors.push(err);
                }
                for error in message.errors {
                    if let Some(err) = error.error {
                        errors.push(err);
                    }
                }
                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(ClientError::Function(errors))
                }
            }
            Err(err) => Err(ClientError::Status(err)),
        }
    }

    async fn get(
        &mut self,
        path: &str,
        view: proto::v1::View,
        _fields: Vec<i32>,
    ) -> Result<Vec<DataEntry>, ClientError> {
        let mut client = proto::v1::val_client::ValClient::with_interceptor(
            self.basic_client.get_channel().await?.clone(),
            self.basic_client.get_auth_interceptor(),
        );

        let get_request = proto::v1::GetRequest {
            entries: vec![proto::v1::EntryRequest {
                path: path.to_string(),
                view: view.into(),
                fields: _fields,
            }],
        };

        match client.get(get_request).await {
            Ok(response) => {
                let message = response.into_inner();
                let mut errors = Vec::new();
                if let Some(err) = message.error {
                    errors.push(err);
                }
                for error in message.errors {
                    if let Some(err) = error.error {
                        errors.push(err);
                    }
                }
                if !errors.is_empty() {
                    Err(ClientError::Function(errors))
                } else {
                    // since there is only one DataEntry in the vector return only the according DataEntry
                    Ok(message.entries.clone())
                }
            }
            Err(err) => Err(ClientError::Status(err)),
        }
    }
}

#[async_trait]
impl common::SDVClientTraitV1 for KuksaClient {
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
        let result = self
            .set_current_values(datapoints.convert_to_v1())
            .await
            .unwrap();
        let converted_result = result.convert_to_sdv();
        Ok(converted_result)
    }

    async fn get_datapoints(
        &mut self,
        paths: Self::PathType,
    ) -> Result<Self::GetResponseType, ClientError> {
        Ok(self
            .get_current_values(paths.convert_to_v1())
            .await
            .unwrap()
            .convert_to_sdv())
    }

    async fn subscribe(
        &mut self,
        _paths: Self::SubscribeType,
    ) -> Result<Self::SubscribeResponseType, ClientError> {
        unimplemented!("Subscribe mechanism has changed. SQL queries not supported anymore")
    }

    async fn set_datapoints(
        &mut self,
        datapoints: Self::UpdateActuationType,
    ) -> Result<Self::ActuateResponseType, ClientError> {
        let result = self
            .set_target_values(datapoints.convert_to_v1())
            .await
            .unwrap();
        let converted_result = result.convert_to_sdv();
        Ok(converted_result)
    }

    async fn get_metadata(
        &mut self,
        paths: Self::PathType,
    ) -> Result<Self::MetadataResponseType, ClientError> {
        Ok(
            common::ClientTraitV1::get_metadata(self, paths.convert_to_v1())
                .await
                .unwrap()
                .convert_to_sdv(),
        )
    }
}

#[async_trait]
impl common::ClientTraitV1 for KuksaClient {
    type SensorUpdateType = types::SensorUpdateTypeV1;
    type UpdateActuationType = types::UpdateActuationTypeV1;
    type PathType = types::PathTypeV1;
    type SubscribeType = Self::PathType;
    type PublishResponseType = types::PublishResponseTypeV1;
    type GetResponseType = types::GetResponseTypeV1;
    type SubscribeResponseType = types::SubscribeResponseTypeV1;
    type ProvideResponseType = types::ProvideResponseTypeV1;
    type ActuateResponseType = types::ActuateResponseTypeV1;
    type MetadataResponseType = types::MetadataResponseTypeV1;

    async fn set_current_values(
        &mut self,
        datapoints: Self::SensorUpdateType,
    ) -> Result<Self::PublishResponseType, ClientError> {
        for (path, datapoint) in datapoints {
            match self
                .set(
                    proto::v1::DataEntry {
                        path: path.clone(),
                        value: Some(datapoint),
                        actuator_target: None,
                        metadata: None,
                    },
                    vec![
                        proto::v1::Field::Value.into(),
                        proto::v1::Field::Path.into(),
                    ],
                )
                .await
            {
                Ok(_) => {
                    continue;
                }
                Err(err) => return Err(err),
            }
        }

        Ok(())
    }

    async fn get_current_values(
        &mut self,
        paths: Self::PathType,
    ) -> Result<Self::GetResponseType, ClientError> {
        let mut get_result = Vec::new();

        for path in paths {
            match self
                .get(
                    &path,
                    proto::v1::View::CurrentValue,
                    vec![
                        proto::v1::Field::Value.into(),
                        proto::v1::Field::Metadata.into(),
                    ],
                )
                .await
            {
                Ok(mut entry) => get_result.append(&mut entry),
                Err(err) => return Err(err),
            }
        }

        Ok(get_result)
    }

    async fn subscribe_target_values(
        &mut self,
        paths: Self::PathType,
    ) -> Result<Self::ProvideResponseType, ClientError> {
        let mut client = proto::v1::val_client::ValClient::with_interceptor(
            self.basic_client.get_channel().await?.clone(),
            self.basic_client.get_auth_interceptor(),
        );
        let mut entries = Vec::new();
        for path in paths {
            entries.push(proto::v1::SubscribeEntry {
                path: path.to_string(),
                view: proto::v1::View::TargetValue.into(),
                fields: vec![proto::v1::Field::ActuatorTarget.into()],
            })
        }

        let req = proto::v1::SubscribeRequest { entries };

        match client.subscribe(req).await {
            Ok(response) => Ok(response.into_inner()),
            Err(err) => Err(ClientError::Status(err)),
        }
    }

    async fn get_target_values(
        &mut self,
        paths: Self::PathType,
    ) -> Result<Self::GetResponseType, ClientError> {
        let mut get_result = Vec::new();

        for path in paths {
            match self
                .get(
                    &path,
                    proto::v1::View::TargetValue,
                    vec![
                        proto::v1::Field::ActuatorTarget.into(),
                        proto::v1::Field::Metadata.into(),
                    ],
                )
                .await
            {
                Ok(mut entry) => get_result.append(&mut entry),
                Err(err) => return Err(err),
            }
        }

        Ok(get_result)
    }

    async fn subscribe_current_values(
        &mut self,
        paths: Self::SubscribeType,
    ) -> Result<Self::SubscribeResponseType, ClientError> {
        let mut client = proto::v1::val_client::ValClient::with_interceptor(
            self.basic_client.get_channel().await?.clone(),
            self.basic_client.get_auth_interceptor(),
        );

        let mut entries = Vec::new();
        for path in paths {
            entries.push(proto::v1::SubscribeEntry {
                path: path.to_string(),
                view: proto::v1::View::CurrentValue.into(),
                fields: vec![
                    proto::v1::Field::Value.into(),
                    proto::v1::Field::Metadata.into(),
                ],
            })
        }

        let req = proto::v1::SubscribeRequest { entries };

        match client.subscribe(req).await {
            Ok(response) => Ok(response.into_inner()),
            Err(err) => Err(ClientError::Status(err)),
        }
    }

    async fn subscribe(
        &mut self,
        paths: Self::SubscribeType,
    ) -> Result<Self::SubscribeResponseType, ClientError> {
        self.subscribe_current_values(paths).await
    }

    async fn set_target_values(
        &mut self,
        datapoints: Self::UpdateActuationType,
    ) -> Result<Self::ActuateResponseType, ClientError> {
        for (path, datapoint) in datapoints {
            match self
                .set(
                    proto::v1::DataEntry {
                        path: path.clone(),
                        value: None,
                        actuator_target: Some(datapoint),
                        metadata: None,
                    },
                    vec![
                        proto::v1::Field::ActuatorTarget.into(),
                        proto::v1::Field::Path.into(),
                    ],
                )
                .await
            {
                Ok(_) => {
                    continue;
                }
                Err(err) => return Err(err),
            }
        }

        Ok(())
    }

    async fn get_metadata(
        &mut self,
        paths: Self::PathType,
    ) -> Result<Self::MetadataResponseType, ClientError> {
        let mut metadata_result = Vec::new();

        for path in paths {
            match self
                .get(
                    &path,
                    proto::v1::View::Metadata,
                    vec![proto::v1::Field::Metadata.into()],
                )
                .await
            {
                Ok(mut entry) => metadata_result.append(&mut entry),
                Err(err) => return Err(err),
            }
        }

        Ok(metadata_result)
    }
}
