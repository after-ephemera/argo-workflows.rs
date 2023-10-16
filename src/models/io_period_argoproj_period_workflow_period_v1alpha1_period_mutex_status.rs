/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMutexStatus : MutexStatus contains which objects hold  mutex locks, and which objects this workflow is waiting on to release locks.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMutexStatus {
    /// Holding is a list of mutexes and their respective objects that are held by mutex lock for this io.argoproj.workflow.v1alpha1.
    #[serde(rename = "holding", skip_serializing_if = "Option::is_none")]
    pub holding: Option<Vec<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMutexHolding>>,
    /// Waiting is a list of mutexes and their respective objects this workflow is waiting for.
    #[serde(rename = "waiting", skip_serializing_if = "Option::is_none")]
    pub waiting: Option<Vec<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMutexHolding>>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMutexStatus {
    /// MutexStatus contains which objects hold  mutex locks, and which objects this workflow is waiting on to release locks.
    pub fn new() -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMutexStatus {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMutexStatus {
            holding: None,
            waiting: None,
        }
    }
}


