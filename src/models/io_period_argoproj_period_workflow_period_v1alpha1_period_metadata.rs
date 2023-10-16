/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMetadata : Pod metdata



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMetadata {
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMetadata {
    /// Pod metdata
    pub fn new() -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMetadata {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMetadata {
            annotations: None,
            labels: None,
        }
    }
}

