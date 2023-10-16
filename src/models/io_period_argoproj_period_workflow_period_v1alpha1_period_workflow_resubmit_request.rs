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
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowResubmitRequest {
    #[serde(rename = "memoized", skip_serializing_if = "Option::is_none")]
    pub memoized: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowResubmitRequest {
    pub fn new() -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowResubmitRequest {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowResubmitRequest {
            memoized: None,
            name: None,
            namespace: None,
            parameters: None,
        }
    }
}

