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
pub struct IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodArgoWorkflowTrigger {
    #[serde(rename = "args", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(rename = "operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTriggerParameter>>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodArtifactLocation>>,
}

impl IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodArgoWorkflowTrigger {
    pub fn new() -> IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodArgoWorkflowTrigger {
        IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodArgoWorkflowTrigger {
            args: None,
            operation: None,
            parameters: None,
            source: None,
        }
    }
}

