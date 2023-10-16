/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodLabelValues : Labels is list of workflow labels



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodLabelValues {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<String>>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodLabelValues {
    /// Labels is list of workflow labels
    pub fn new() -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodLabelValues {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodLabelValues {
            items: None,
        }
    }
}


