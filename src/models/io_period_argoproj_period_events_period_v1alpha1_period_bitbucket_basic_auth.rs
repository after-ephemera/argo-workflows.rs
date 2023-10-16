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
pub struct IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketBasicAuth {
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector>>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector>>,
}

impl IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketBasicAuth {
    pub fn new() -> IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketBasicAuth {
        IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketBasicAuth {
            password: None,
            username: None,
        }
    }
}

