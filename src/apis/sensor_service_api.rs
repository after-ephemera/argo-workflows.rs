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


/// struct for typed errors of method [`sensor_service_create_sensor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SensorServiceCreateSensorError {
    DefaultResponse(crate::models::GrpcPeriodGatewayPeriodRuntimePeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sensor_service_delete_sensor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SensorServiceDeleteSensorError {
    DefaultResponse(crate::models::GrpcPeriodGatewayPeriodRuntimePeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sensor_service_get_sensor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SensorServiceGetSensorError {
    DefaultResponse(crate::models::GrpcPeriodGatewayPeriodRuntimePeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sensor_service_list_sensors`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SensorServiceListSensorsError {
    DefaultResponse(crate::models::GrpcPeriodGatewayPeriodRuntimePeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sensor_service_sensors_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SensorServiceSensorsLogsError {
    DefaultResponse(crate::models::GrpcPeriodGatewayPeriodRuntimePeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sensor_service_update_sensor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SensorServiceUpdateSensorError {
    DefaultResponse(crate::models::GrpcPeriodGatewayPeriodRuntimePeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sensor_service_watch_sensors`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SensorServiceWatchSensorsError {
    DefaultResponse(crate::models::GrpcPeriodGatewayPeriodRuntimePeriodError),
    UnknownValue(serde_json::Value),
}


pub async fn sensor_service_create_sensor(configuration: &configuration::Configuration, namespace: &str, body: crate::models::SensorPeriodCreateSensorRequest) -> Result<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSensor, Error<SensorServiceCreateSensorError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/sensors/{namespace}", local_var_configuration.base_path, namespace=crate::apis::urlencode(namespace));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SensorServiceCreateSensorError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn sensor_service_delete_sensor(configuration: &configuration::Configuration, namespace: &str, name: &str, delete_options_period_grace_period_seconds: Option<&str>, delete_options_period_preconditions_period_uid: Option<&str>, delete_options_period_preconditions_period_resource_version: Option<&str>, delete_options_period_orphan_dependents: Option<bool>, delete_options_period_propagation_policy: Option<&str>, delete_options_period_dry_run: Option<Vec<String>>) -> Result<serde_json::Value, Error<SensorServiceDeleteSensorError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/sensors/{namespace}/{name}", local_var_configuration.base_path, namespace=crate::apis::urlencode(namespace), name=crate::apis::urlencode(name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = delete_options_period_grace_period_seconds {
        local_var_req_builder = local_var_req_builder.query(&[("deleteOptions.gracePeriodSeconds", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = delete_options_period_preconditions_period_uid {
        local_var_req_builder = local_var_req_builder.query(&[("deleteOptions.preconditions.uid", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = delete_options_period_preconditions_period_resource_version {
        local_var_req_builder = local_var_req_builder.query(&[("deleteOptions.preconditions.resourceVersion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = delete_options_period_orphan_dependents {
        local_var_req_builder = local_var_req_builder.query(&[("deleteOptions.orphanDependents", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = delete_options_period_propagation_policy {
        local_var_req_builder = local_var_req_builder.query(&[("deleteOptions.propagationPolicy", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = delete_options_period_dry_run {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("deleteOptions.dryRun".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("deleteOptions.dryRun", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
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
        let local_var_entity: Option<SensorServiceDeleteSensorError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn sensor_service_get_sensor(configuration: &configuration::Configuration, namespace: &str, name: &str, get_options_period_resource_version: Option<&str>) -> Result<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSensor, Error<SensorServiceGetSensorError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/sensors/{namespace}/{name}", local_var_configuration.base_path, namespace=crate::apis::urlencode(namespace), name=crate::apis::urlencode(name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = get_options_period_resource_version {
        local_var_req_builder = local_var_req_builder.query(&[("getOptions.resourceVersion", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<SensorServiceGetSensorError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn sensor_service_list_sensors(configuration: &configuration::Configuration, namespace: &str, list_options_period_label_selector: Option<&str>, list_options_period_field_selector: Option<&str>, list_options_period_watch: Option<bool>, list_options_period_allow_watch_bookmarks: Option<bool>, list_options_period_resource_version: Option<&str>, list_options_period_resource_version_match: Option<&str>, list_options_period_timeout_seconds: Option<&str>, list_options_period_limit: Option<&str>, list_options_period_continue: Option<&str>) -> Result<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSensorList, Error<SensorServiceListSensorsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/sensors/{namespace}", local_var_configuration.base_path, namespace=crate::apis::urlencode(namespace));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = list_options_period_label_selector {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.labelSelector", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_period_field_selector {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.fieldSelector", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_period_watch {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.watch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_period_allow_watch_bookmarks {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.allowWatchBookmarks", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_period_resource_version {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.resourceVersion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_period_resource_version_match {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.resourceVersionMatch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_period_timeout_seconds {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.timeoutSeconds", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_period_limit {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_period_continue {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.continue", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<SensorServiceListSensorsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn sensor_service_sensors_logs(configuration: &configuration::Configuration, namespace: &str, name: Option<&str>, trigger_name: Option<&str>, grep: Option<&str>, pod_log_options_period_container: Option<&str>, pod_log_options_period_follow: Option<bool>, pod_log_options_period_previous: Option<bool>, pod_log_options_period_since_seconds: Option<&str>, pod_log_options_period_since_time_period_seconds: Option<&str>, pod_log_options_period_since_time_period_nanos: Option<i32>, pod_log_options_period_timestamps: Option<bool>, pod_log_options_period_tail_lines: Option<&str>, pod_log_options_period_limit_bytes: Option<&str>, pod_log_options_period_insecure_skip_tls_verify_backend: Option<bool>) -> Result<crate::models::StreamResultOfSensorLogEntry, Error<SensorServiceSensorsLogsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/stream/sensors/{namespace}/logs", local_var_configuration.base_path, namespace=crate::apis::urlencode(namespace));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = name {
        local_var_req_builder = local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = trigger_name {
        local_var_req_builder = local_var_req_builder.query(&[("triggerName", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = grep {
        local_var_req_builder = local_var_req_builder.query(&[("grep", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pod_log_options_period_container {
        local_var_req_builder = local_var_req_builder.query(&[("podLogOptions.container", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pod_log_options_period_follow {
        local_var_req_builder = local_var_req_builder.query(&[("podLogOptions.follow", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pod_log_options_period_previous {
        local_var_req_builder = local_var_req_builder.query(&[("podLogOptions.previous", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pod_log_options_period_since_seconds {
        local_var_req_builder = local_var_req_builder.query(&[("podLogOptions.sinceSeconds", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pod_log_options_period_since_time_period_seconds {
        local_var_req_builder = local_var_req_builder.query(&[("podLogOptions.sinceTime.seconds", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pod_log_options_period_since_time_period_nanos {
        local_var_req_builder = local_var_req_builder.query(&[("podLogOptions.sinceTime.nanos", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pod_log_options_period_timestamps {
        local_var_req_builder = local_var_req_builder.query(&[("podLogOptions.timestamps", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pod_log_options_period_tail_lines {
        local_var_req_builder = local_var_req_builder.query(&[("podLogOptions.tailLines", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pod_log_options_period_limit_bytes {
        local_var_req_builder = local_var_req_builder.query(&[("podLogOptions.limitBytes", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pod_log_options_period_insecure_skip_tls_verify_backend {
        local_var_req_builder = local_var_req_builder.query(&[("podLogOptions.insecureSkipTLSVerifyBackend", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<SensorServiceSensorsLogsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn sensor_service_update_sensor(configuration: &configuration::Configuration, namespace: &str, name: &str, body: crate::models::SensorPeriodUpdateSensorRequest) -> Result<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSensor, Error<SensorServiceUpdateSensorError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/sensors/{namespace}/{name}", local_var_configuration.base_path, namespace=crate::apis::urlencode(namespace), name=crate::apis::urlencode(name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SensorServiceUpdateSensorError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn sensor_service_watch_sensors(configuration: &configuration::Configuration, namespace: &str, list_options_period_label_selector: Option<&str>, list_options_period_field_selector: Option<&str>, list_options_period_watch: Option<bool>, list_options_period_allow_watch_bookmarks: Option<bool>, list_options_period_resource_version: Option<&str>, list_options_period_resource_version_match: Option<&str>, list_options_period_timeout_seconds: Option<&str>, list_options_period_limit: Option<&str>, list_options_period_continue: Option<&str>) -> Result<crate::models::StreamResultOfSensorSensorWatchEvent, Error<SensorServiceWatchSensorsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/stream/sensors/{namespace}", local_var_configuration.base_path, namespace=crate::apis::urlencode(namespace));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = list_options_period_label_selector {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.labelSelector", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_period_field_selector {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.fieldSelector", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_period_watch {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.watch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_period_allow_watch_bookmarks {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.allowWatchBookmarks", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_period_resource_version {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.resourceVersion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_period_resource_version_match {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.resourceVersionMatch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_period_timeout_seconds {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.timeoutSeconds", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_period_limit {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_period_continue {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.continue", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<SensorServiceWatchSensorsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

