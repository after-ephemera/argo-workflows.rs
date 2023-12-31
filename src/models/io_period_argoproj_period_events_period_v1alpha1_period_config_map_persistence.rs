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
pub struct IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodConfigMapPersistence {
    #[serde(rename = "createIfNotExist", skip_serializing_if = "Option::is_none")]
    pub create_if_not_exist: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodConfigMapPersistence {
    pub fn new() -> IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodConfigMapPersistence {
        IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodConfigMapPersistence {
            create_if_not_exist: None,
            name: None,
        }
    }
}


