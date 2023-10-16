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
pub struct IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodStandardK8STrigger {
    #[serde(rename = "liveObject", skip_serializing_if = "Option::is_none")]
    pub live_object: Option<bool>,
    #[serde(rename = "operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    /// Parameters is the list of parameters that is applied to resolved K8s trigger object.
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTriggerParameter>>,
    #[serde(rename = "patchStrategy", skip_serializing_if = "Option::is_none")]
    pub patch_strategy: Option<String>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodArtifactLocation>>,
}

impl IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodStandardK8STrigger {
    pub fn new() -> IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodStandardK8STrigger {
        IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodStandardK8STrigger {
            live_object: None,
            operation: None,
            parameters: None,
            patch_strategy: None,
            source: None,
        }
    }
}


