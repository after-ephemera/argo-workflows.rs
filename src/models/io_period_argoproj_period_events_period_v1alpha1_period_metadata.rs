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
pub struct IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodMetadata {
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
}

impl IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodMetadata {
    pub fn new() -> IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodMetadata {
        IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodMetadata {
            annotations: None,
            labels: None,
        }
    }
}


