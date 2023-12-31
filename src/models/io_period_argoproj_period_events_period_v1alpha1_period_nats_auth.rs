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
pub struct IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodNatsAuth {
    #[serde(rename = "basic", skip_serializing_if = "Option::is_none")]
    pub basic: Option<Box<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBasicAuth>>,
    #[serde(rename = "credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector>>,
    #[serde(rename = "nkey", skip_serializing_if = "Option::is_none")]
    pub nkey: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector>>,
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector>>,
}

impl IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodNatsAuth {
    pub fn new() -> IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodNatsAuth {
        IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodNatsAuth {
            basic: None,
            credential: None,
            nkey: None,
            token: None,
        }
    }
}


