/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`artifact_service_get_artifact_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArtifactServiceGetArtifactFileError {
    DefaultResponse(crate::models::GrpcPeriodGatewayPeriodRuntimePeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`artifact_service_get_input_artifact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArtifactServiceGetInputArtifactError {
    DefaultResponse(crate::models::GrpcPeriodGatewayPeriodRuntimePeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`artifact_service_get_input_artifact_by_uid`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArtifactServiceGetInputArtifactByUidError {
    DefaultResponse(crate::models::GrpcPeriodGatewayPeriodRuntimePeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`artifact_service_get_output_artifact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArtifactServiceGetOutputArtifactError {
    DefaultResponse(crate::models::GrpcPeriodGatewayPeriodRuntimePeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`artifact_service_get_output_artifact_by_uid`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArtifactServiceGetOutputArtifactByUidError {
    DefaultResponse(crate::models::GrpcPeriodGatewayPeriodRuntimePeriodError),
    UnknownValue(serde_json::Value),
}


pub async fn artifact_service_get_artifact_file(configuration: &configuration::Configuration, namespace: &str, id_discriminator: &str, id: &str, node_id: &str, artifact_name: &str, artifact_discriminator: &str) -> Result<std::path::PathBuf, Error<ArtifactServiceGetArtifactFileError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/artifact-files/{namespace}/{idDiscriminator}/{id}/{nodeId}/{artifactDiscriminator}/{artifactName}", local_var_configuration.base_path, namespace=crate::apis::urlencode(namespace), idDiscriminator=crate::apis::urlencode(id_discriminator), id=crate::apis::urlencode(id), nodeId=crate::apis::urlencode(node_id), artifactName=crate::apis::urlencode(artifact_name), artifactDiscriminator=crate::apis::urlencode(artifact_discriminator));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ArtifactServiceGetArtifactFileError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn artifact_service_get_input_artifact(configuration: &configuration::Configuration, namespace: &str, name: &str, node_id: &str, artifact_name: &str) -> Result<std::path::PathBuf, Error<ArtifactServiceGetInputArtifactError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/input-artifacts/{namespace}/{name}/{nodeId}/{artifactName}", local_var_configuration.base_path, namespace=crate::apis::urlencode(namespace), name=crate::apis::urlencode(name), nodeId=crate::apis::urlencode(node_id), artifactName=crate::apis::urlencode(artifact_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ArtifactServiceGetInputArtifactError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn artifact_service_get_input_artifact_by_uid(configuration: &configuration::Configuration, uid: &str, node_id: &str, artifact_name: &str) -> Result<std::path::PathBuf, Error<ArtifactServiceGetInputArtifactByUidError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/input-artifacts-by-uid/{uid}/{nodeId}/{artifactName}", local_var_configuration.base_path, uid=crate::apis::urlencode(uid), nodeId=crate::apis::urlencode(node_id), artifactName=crate::apis::urlencode(artifact_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ArtifactServiceGetInputArtifactByUidError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn artifact_service_get_output_artifact(configuration: &configuration::Configuration, namespace: &str, name: &str, node_id: &str, artifact_name: &str) -> Result<std::path::PathBuf, Error<ArtifactServiceGetOutputArtifactError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/artifacts/{namespace}/{name}/{nodeId}/{artifactName}", local_var_configuration.base_path, namespace=crate::apis::urlencode(namespace), name=crate::apis::urlencode(name), nodeId=crate::apis::urlencode(node_id), artifactName=crate::apis::urlencode(artifact_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ArtifactServiceGetOutputArtifactError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn artifact_service_get_output_artifact_by_uid(configuration: &configuration::Configuration, uid: &str, node_id: &str, artifact_name: &str) -> Result<std::path::PathBuf, Error<ArtifactServiceGetOutputArtifactByUidError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/artifacts-by-uid/{uid}/{nodeId}/{artifactName}", local_var_configuration.base_path, uid=crate::apis::urlencode(uid), nodeId=crate::apis::urlencode(node_id), artifactName=crate::apis::urlencode(artifact_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ArtifactServiceGetOutputArtifactByUidError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

