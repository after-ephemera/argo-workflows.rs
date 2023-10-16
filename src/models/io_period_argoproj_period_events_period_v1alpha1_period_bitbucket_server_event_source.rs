/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketServerEventSource {
    #[serde(rename = "accessToken", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector>>,
    #[serde(rename = "bitbucketserverBaseURL", skip_serializing_if = "Option::is_none")]
    pub bitbucketserver_base_url: Option<String>,
    #[serde(rename = "deleteHookOnFinish", skip_serializing_if = "Option::is_none")]
    pub delete_hook_on_finish: Option<bool>,
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodEventSourceFilter>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "projectKey", skip_serializing_if = "Option::is_none")]
    pub project_key: Option<String>,
    #[serde(rename = "repositories", skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketServerRepository>>,
    #[serde(rename = "repositorySlug", skip_serializing_if = "Option::is_none")]
    pub repository_slug: Option<String>,
    #[serde(rename = "webhook", skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Box<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodWebhookContext>>,
    #[serde(rename = "webhookSecret", skip_serializing_if = "Option::is_none")]
    pub webhook_secret: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector>>,
}

impl IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketServerEventSource {
    pub fn new() -> IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketServerEventSource {
        IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketServerEventSource {
            access_token: None,
            bitbucketserver_base_url: None,
            delete_hook_on_finish: None,
            events: None,
            filter: None,
            metadata: None,
            project_key: None,
            repositories: None,
            repository_slug: None,
            webhook: None,
            webhook_secret: None,
        }
    }
}


