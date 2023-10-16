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
pub struct IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodRateLimit {
    #[serde(rename = "requestsPerUnit", skip_serializing_if = "Option::is_none")]
    pub requests_per_unit: Option<i32>,
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

impl IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodRateLimit {
    pub fn new() -> IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodRateLimit {
        IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodRateLimit {
            requests_per_unit: None,
            unit: None,
        }
    }
}


