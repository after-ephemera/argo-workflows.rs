/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodRetryStrategy : RetryStrategy provides controls on how to retry a workflow step



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodRetryStrategy {
    #[serde(rename = "affinity", skip_serializing_if = "Option::is_none")]
    pub affinity: Option<Box<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodRetryAffinity>>,
    #[serde(rename = "backoff", skip_serializing_if = "Option::is_none")]
    pub backoff: Option<Box<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodBackoff>>,
    /// Expression is a condition expression for when a node will be retried. If it evaluates to false, the node will not be retried and the retry strategy will be ignored
    #[serde(rename = "expression", skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// RetryPolicy is a policy of NodePhase statuses that will be retried
    #[serde(rename = "retryPolicy", skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<String>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodRetryStrategy {
    /// RetryStrategy provides controls on how to retry a workflow step
    pub fn new() -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodRetryStrategy {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodRetryStrategy {
            affinity: None,
            backoff: None,
            expression: None,
            limit: None,
            retry_policy: None,
        }
    }
}


