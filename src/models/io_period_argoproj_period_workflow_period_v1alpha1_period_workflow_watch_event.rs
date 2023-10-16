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
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowWatchEvent {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<Box<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflow>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowWatchEvent {
    pub fn new() -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowWatchEvent {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowWatchEvent {
            object: None,
            r#type: None,
        }
    }
}

