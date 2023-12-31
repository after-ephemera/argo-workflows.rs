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
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowSetRequest {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "nodeFieldSelector", skip_serializing_if = "Option::is_none")]
    pub node_field_selector: Option<String>,
    #[serde(rename = "outputParameters", skip_serializing_if = "Option::is_none")]
    pub output_parameters: Option<String>,
    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowSetRequest {
    pub fn new() -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowSetRequest {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowSetRequest {
            message: None,
            name: None,
            namespace: None,
            node_field_selector: None,
            output_parameters: None,
            phase: None,
        }
    }
}


