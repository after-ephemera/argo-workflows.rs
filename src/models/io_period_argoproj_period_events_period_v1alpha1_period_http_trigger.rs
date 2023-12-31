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
pub struct IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodHttpTrigger {
    #[serde(rename = "basicAuth", skip_serializing_if = "Option::is_none")]
    pub basic_auth: Option<Box<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBasicAuth>>,
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Parameters is the list of key-value extracted from event's payload that are applied to the HTTP trigger resource.
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTriggerParameter>>,
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Vec<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTriggerParameter>>,
    #[serde(rename = "secureHeaders", skip_serializing_if = "Option::is_none")]
    pub secure_headers: Option<Vec<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSecureHeader>>,
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(rename = "tls", skip_serializing_if = "Option::is_none")]
    pub tls: Option<Box<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTlsConfig>>,
    /// URL refers to the URL to send HTTP request to.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodHttpTrigger {
    pub fn new() -> IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodHttpTrigger {
        IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodHttpTrigger {
            basic_auth: None,
            headers: None,
            method: None,
            parameters: None,
            payload: None,
            secure_headers: None,
            timeout: None,
            tls: None,
            url: None,
        }
    }
}


