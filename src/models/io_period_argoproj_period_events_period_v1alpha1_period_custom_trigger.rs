/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodCustomTrigger : CustomTrigger refers to the specification of the custom trigger.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodCustomTrigger {
    #[serde(rename = "certSecret", skip_serializing_if = "Option::is_none")]
    pub cert_secret: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector>>,
    /// Parameters is the list of parameters that is applied to resolved custom trigger trigger object.
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTriggerParameter>>,
    /// Payload is the list of key-value extracted from an event payload to construct the request payload.
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Vec<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTriggerParameter>>,
    #[serde(rename = "secure", skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    /// ServerNameOverride for the secure connection between sensor and custom trigger gRPC server.
    #[serde(rename = "serverNameOverride", skip_serializing_if = "Option::is_none")]
    pub server_name_override: Option<String>,
    #[serde(rename = "serverURL", skip_serializing_if = "Option::is_none")]
    pub server_url: Option<String>,
    /// Spec is the custom trigger resource specification that custom trigger gRPC server knows how to interpret.
    #[serde(rename = "spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<::std::collections::HashMap<String, String>>,
}

impl IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodCustomTrigger {
    /// CustomTrigger refers to the specification of the custom trigger.
    pub fn new() -> IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodCustomTrigger {
        IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodCustomTrigger {
            cert_secret: None,
            parameters: None,
            payload: None,
            secure: None,
            server_name_override: None,
            server_url: None,
            spec: None,
        }
    }
}


