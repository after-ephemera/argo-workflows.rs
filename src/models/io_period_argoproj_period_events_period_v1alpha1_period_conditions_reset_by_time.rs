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
pub struct IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodConditionsResetByTime {
    #[serde(rename = "cron", skip_serializing_if = "Option::is_none")]
    pub cron: Option<String>,
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodConditionsResetByTime {
    pub fn new() -> IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodConditionsResetByTime {
        IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodConditionsResetByTime {
            cron: None,
            timezone: None,
        }
    }
}


