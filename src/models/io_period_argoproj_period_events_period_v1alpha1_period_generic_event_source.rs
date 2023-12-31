/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodGenericEventSource : GenericEventSource refers to a generic event source. It can be used to implement a custom event source.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodGenericEventSource {
    #[serde(rename = "authSecret", skip_serializing_if = "Option::is_none")]
    pub auth_secret: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector>>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodEventSourceFilter>>,
    /// Insecure determines the type of connection.
    #[serde(rename = "insecure", skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    #[serde(rename = "jsonBody", skip_serializing_if = "Option::is_none")]
    pub json_body: Option<bool>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    /// URL of the gRPC server that implements the event source.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodGenericEventSource {
    /// GenericEventSource refers to a generic event source. It can be used to implement a custom event source.
    pub fn new() -> IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodGenericEventSource {
        IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodGenericEventSource {
            auth_secret: None,
            config: None,
            filter: None,
            insecure: None,
            json_body: None,
            metadata: None,
            url: None,
        }
    }
}


