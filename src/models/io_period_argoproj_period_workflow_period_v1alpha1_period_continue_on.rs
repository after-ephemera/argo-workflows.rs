/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodContinueOn : ContinueOn defines if a workflow should continue even if a task or step fails/errors. It can be specified if the workflow should continue when the pod errors, fails or both.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodContinueOn {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>,
    #[serde(rename = "failed", skip_serializing_if = "Option::is_none")]
    pub failed: Option<bool>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodContinueOn {
    /// ContinueOn defines if a workflow should continue even if a task or step fails/errors. It can be specified if the workflow should continue when the pod errors, fails or both.
    pub fn new() -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodContinueOn {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodContinueOn {
            error: None,
            failed: None,
        }
    }
}


