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
pub struct IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketEventSource {
    #[serde(rename = "auth", skip_serializing_if = "Option::is_none")]
    pub auth: Option<Box<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketAuth>>,
    #[serde(rename = "deleteHookOnFinish", skip_serializing_if = "Option::is_none")]
    pub delete_hook_on_finish: Option<bool>,
    /// Events this webhook is subscribed to.
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodEventSourceFilter>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "projectKey", skip_serializing_if = "Option::is_none")]
    pub project_key: Option<String>,
    #[serde(rename = "repositories", skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketRepository>>,
    #[serde(rename = "repositorySlug", skip_serializing_if = "Option::is_none")]
    pub repository_slug: Option<String>,
    #[serde(rename = "webhook", skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Box<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodWebhookContext>>,
}

impl IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketEventSource {
    pub fn new() -> IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketEventSource {
        IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketEventSource {
            auth: None,
            delete_hook_on_finish: None,
            events: None,
            filter: None,
            metadata: None,
            owner: None,
            project_key: None,
            repositories: None,
            repository_slug: None,
            webhook: None,
        }
    }
}


