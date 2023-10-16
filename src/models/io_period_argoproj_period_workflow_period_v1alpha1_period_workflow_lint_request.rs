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
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowLintRequest {
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "workflow", skip_serializing_if = "Option::is_none")]
    pub workflow: Option<Box<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflow>>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowLintRequest {
    pub fn new() -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowLintRequest {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowLintRequest {
            namespace: None,
            workflow: None,
        }
    }
}

