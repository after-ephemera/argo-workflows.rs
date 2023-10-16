/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSensorStatus : SensorStatus contains information about the status of a sensor.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSensorStatus {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodStatus>>,
}

impl IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSensorStatus {
    /// SensorStatus contains information about the status of a sensor.
    pub fn new() -> IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSensorStatus {
        IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSensorStatus {
            status: None,
        }
    }
}


