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
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowSubmitRequest {
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "resourceKind", skip_serializing_if = "Option::is_none")]
    pub resource_kind: Option<String>,
    #[serde(rename = "resourceName", skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "submitOptions", skip_serializing_if = "Option::is_none")]
    pub submit_options: Option<Box<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodSubmitOpts>>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowSubmitRequest {
    pub fn new() -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowSubmitRequest {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowSubmitRequest {
            namespace: None,
            resource_kind: None,
            resource_name: None,
            submit_options: None,
        }
    }
}


