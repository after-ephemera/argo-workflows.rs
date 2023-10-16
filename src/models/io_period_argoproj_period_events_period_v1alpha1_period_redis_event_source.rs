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
pub struct IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodRedisEventSource {
    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(rename = "db", skip_serializing_if = "Option::is_none")]
    pub db: Option<i32>,
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodEventSourceFilter>>,
    #[serde(rename = "hostAddress", skip_serializing_if = "Option::is_none")]
    pub host_address: Option<String>,
    #[serde(rename = "jsonBody", skip_serializing_if = "Option::is_none")]
    pub json_body: Option<bool>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector>>,
    #[serde(rename = "tls", skip_serializing_if = "Option::is_none")]
    pub tls: Option<Box<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTlsConfig>>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodRedisEventSource {
    pub fn new() -> IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodRedisEventSource {
        IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodRedisEventSource {
            channels: None,
            db: None,
            filter: None,
            host_address: None,
            json_body: None,
            metadata: None,
            namespace: None,
            password: None,
            tls: None,
            username: None,
        }
    }
}

