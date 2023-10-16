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
pub struct IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSecureHeader {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "valueFrom", skip_serializing_if = "Option::is_none")]
    pub value_from: Option<Box<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodValueFromSource>>,
}

impl IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSecureHeader {
    pub fn new() -> IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSecureHeader {
        IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSecureHeader {
            name: None,
            value_from: None,
        }
    }
}

