/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMetricLabel : MetricLabel is a single label for a prometheus metric



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMetricLabel {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMetricLabel {
    /// MetricLabel is a single label for a prometheus metric
    pub fn new(key: String, value: String) -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMetricLabel {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMetricLabel {
            key,
            value,
        }
    }
}

