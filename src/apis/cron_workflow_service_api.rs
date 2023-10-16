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


/// struct for typed errors of method [`cron_workflow_service_create_cron_workflow`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CronWorkflowServiceCreateCronWorkflowError {
    DefaultResponse(crate::models::GrpcPeriodGatewayPeriodRuntimePeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cron_workflow_service_delete_cron_workflow`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CronWorkflowServiceDeleteCronWorkflowError {
    DefaultResponse(crate::models::GrpcPeriodGatewayPeriodRuntimePeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cron_workflow_service_get_cron_workflow`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CronWorkflowServiceGetCronWorkflowError {
    DefaultResponse(crate::models::GrpcPeriodGatewayPeriodRuntimePeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cron_workflow_service_lint_cron_workflow`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CronWorkflowServiceLintCronWorkflowError {
    DefaultResponse(crate::models::GrpcPeriodGatewayPeriodRuntimePeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cron_workflow_service_list_cron_workflows`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CronWorkflowServiceListCronWorkflowsError {
    DefaultResponse(crate::models::GrpcPeriodGatewayPeriodRuntimePeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cron_workflow_service_resume_cron_workflow`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CronWorkflowServiceResumeCronWorkflowError {
    DefaultResponse(crate::models::GrpcPeriodGatewayPeriodRuntimePeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cron_workflow_service_suspend_cron_workflow`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CronWorkflowServiceSuspendCronWorkflowError {
    DefaultResponse(crate::models::GrpcPeriodGatewayPeriodRuntimePeriodError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cron_workflow_service_update_cron_workflow`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CronWorkflowServiceUpdateCronWorkflowError {
    DefaultResponse(crate::models::GrpcPeriodGatewayPeriodRuntimePeriodError),
    UnknownValue(serde_json::Value),
}


pub async fn cron_workflow_service_create_cron_workflow(configuration: &configuration::Configuration, namespace: &str, body: crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodCreateCronWorkflowRequest) -> Result<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodCronWorkflow, Error<CronWorkflowServiceCreateCronWorkflowError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/cron-workflows/{namespace}", local_var_configuration.base_path, namespace=crate::apis::urlencode(namespace));
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
        let local_var_entity: Option<CronWorkflowServiceCreateCronWorkflowError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn cron_workflow_service_delete_cron_workflow(configuration: &configuration::Configuration, namespace: &str, name: &str, delete_options_period_grace_period_seconds: Option<&str>, delete_options_period_preconditions_period_uid: Option<&str>, delete_options_period_preconditions_period_resource_version: Option<&str>, delete_options_period_orphan_dependents: Option<bool>, delete_options_period_propagation_policy: Option<&str>, delete_options_period_dry_run: Option<Vec<String>>) -> Result<serde_json::Value, Error<CronWorkflowServiceDeleteCronWorkflowError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/cron-workflows/{namespace}/{name}", local_var_configuration.base_path, namespace=crate::apis::urlencode(namespace), name=crate::apis::urlencode(name));
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
        let local_var_entity: Option<CronWorkflowServiceDeleteCronWorkflowError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn cron_workflow_service_get_cron_workflow(configuration: &configuration::Configuration, namespace: &str, name: &str, get_options_period_resource_version: Option<&str>) -> Result<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodCronWorkflow, Error<CronWorkflowServiceGetCronWorkflowError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/cron-workflows/{namespace}/{name}", local_var_configuration.base_path, namespace=crate::apis::urlencode(namespace), name=crate::apis::urlencode(name));
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
        let local_var_entity: Option<CronWorkflowServiceGetCronWorkflowError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn cron_workflow_service_lint_cron_workflow(configuration: &configuration::Configuration, namespace: &str, body: crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodLintCronWorkflowRequest) -> Result<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodCronWorkflow, Error<CronWorkflowServiceLintCronWorkflowError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/cron-workflows/{namespace}/lint", local_var_configuration.base_path, namespace=crate::apis::urlencode(namespace));
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
        let local_var_entity: Option<CronWorkflowServiceLintCronWorkflowError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn cron_workflow_service_list_cron_workflows(configuration: &configuration::Configuration, namespace: &str, list_options_period_label_selector: Option<&str>, list_options_period_field_selector: Option<&str>, list_options_period_watch: Option<bool>, list_options_period_allow_watch_bookmarks: Option<bool>, list_options_period_resource_version: Option<&str>, list_options_period_resource_version_match: Option<&str>, list_options_period_timeout_seconds: Option<&str>, list_options_period_limit: Option<&str>, list_options_period_continue: Option<&str>) -> Result<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodCronWorkflowList, Error<CronWorkflowServiceListCronWorkflowsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/cron-workflows/{namespace}", local_var_configuration.base_path, namespace=crate::apis::urlencode(namespace));
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
        let local_var_entity: Option<CronWorkflowServiceListCronWorkflowsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn cron_workflow_service_resume_cron_workflow(configuration: &configuration::Configuration, namespace: &str, name: &str, body: crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodCronWorkflowResumeRequest) -> Result<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodCronWorkflow, Error<CronWorkflowServiceResumeCronWorkflowError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/cron-workflows/{namespace}/{name}/resume", local_var_configuration.base_path, namespace=crate::apis::urlencode(namespace), name=crate::apis::urlencode(name));
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
        let local_var_entity: Option<CronWorkflowServiceResumeCronWorkflowError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn cron_workflow_service_suspend_cron_workflow(configuration: &configuration::Configuration, namespace: &str, name: &str, body: crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodCronWorkflowSuspendRequest) -> Result<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodCronWorkflow, Error<CronWorkflowServiceSuspendCronWorkflowError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/cron-workflows/{namespace}/{name}/suspend", local_var_configuration.base_path, namespace=crate::apis::urlencode(namespace), name=crate::apis::urlencode(name));
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
        let local_var_entity: Option<CronWorkflowServiceSuspendCronWorkflowError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn cron_workflow_service_update_cron_workflow(configuration: &configuration::Configuration, namespace: &str, name: &str, body: crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodUpdateCronWorkflowRequest) -> Result<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodCronWorkflow, Error<CronWorkflowServiceUpdateCronWorkflowError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/cron-workflows/{namespace}/{name}", local_var_configuration.base_path, namespace=crate::apis::urlencode(namespace), name=crate::apis::urlencode(name));
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
        let local_var_entity: Option<CronWorkflowServiceUpdateCronWorkflowError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

