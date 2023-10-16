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
pub struct IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodOwnedRepositories {
    #[serde(rename = "names", skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

impl IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodOwnedRepositories {
    pub fn new() -> IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodOwnedRepositories {
        IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodOwnedRepositories {
            names: None,
            owner: None,
        }
    }
}


