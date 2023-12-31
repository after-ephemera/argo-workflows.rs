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
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodSemaphoreStatus {
    /// Holding stores the list of resource acquired synchronization lock for workflows.
    #[serde(rename = "holding", skip_serializing_if = "Option::is_none")]
    pub holding: Option<Vec<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodSemaphoreHolding>>,
    /// Waiting indicates the list of current synchronization lock holders.
    #[serde(rename = "waiting", skip_serializing_if = "Option::is_none")]
    pub waiting: Option<Vec<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodSemaphoreHolding>>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodSemaphoreStatus {
    pub fn new() -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodSemaphoreStatus {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodSemaphoreStatus {
            holding: None,
            waiting: None,
        }
    }
}


