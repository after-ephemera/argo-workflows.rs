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
pub struct IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSensorSpec {
    /// Dependencies is a list of the events that this sensor is dependent on.
    #[serde(rename = "dependencies", skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodEventDependency>>,
    /// ErrorOnFailedRound if set to true, marks sensor state as `error` if the previous trigger round fails. Once sensor state is set to `error`, no further triggers will be processed.
    #[serde(rename = "errorOnFailedRound", skip_serializing_if = "Option::is_none")]
    pub error_on_failed_round: Option<bool>,
    #[serde(rename = "eventBusName", skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "replicas", skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<Box<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTemplate>>,
    /// Triggers is a list of the things that this sensor evokes. These are the outputs from this sensor.
    #[serde(rename = "triggers", skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTrigger>>,
}

impl IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSensorSpec {
    pub fn new() -> IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSensorSpec {
        IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSensorSpec {
            dependencies: None,
            error_on_failed_round: None,
            event_bus_name: None,
            replicas: None,
            template: None,
            triggers: None,
        }
    }
}


