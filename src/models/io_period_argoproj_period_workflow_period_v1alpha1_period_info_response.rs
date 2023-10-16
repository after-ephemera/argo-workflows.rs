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
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodInfoResponse {
    #[serde(rename = "columns", skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodColumn>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodLink>>,
    #[serde(rename = "managedNamespace", skip_serializing_if = "Option::is_none")]
    pub managed_namespace: Option<String>,
    #[serde(rename = "modals", skip_serializing_if = "Option::is_none")]
    pub modals: Option<::std::collections::HashMap<String, bool>>,
    #[serde(rename = "navColor", skip_serializing_if = "Option::is_none")]
    pub nav_color: Option<String>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodInfoResponse {
    pub fn new() -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodInfoResponse {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodInfoResponse {
            columns: None,
            links: None,
            managed_namespace: None,
            modals: None,
            nav_color: None,
        }
    }
}


