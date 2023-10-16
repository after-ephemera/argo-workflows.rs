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
pub struct IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSnsEventSource {
    #[serde(rename = "accessKey", skip_serializing_if = "Option::is_none")]
    pub access_key: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector>>,
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodEventSourceFilter>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "roleARN", skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "secretKey", skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector>>,
    #[serde(rename = "topicArn", skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(rename = "validateSignature", skip_serializing_if = "Option::is_none")]
    pub validate_signature: Option<bool>,
    #[serde(rename = "webhook", skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Box<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodWebhookContext>>,
}

impl IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSnsEventSource {
    pub fn new() -> IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSnsEventSource {
        IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSnsEventSource {
            access_key: None,
            endpoint: None,
            filter: None,
            metadata: None,
            region: None,
            role_arn: None,
            secret_key: None,
            topic_arn: None,
            validate_signature: None,
            webhook: None,
        }
    }
}


