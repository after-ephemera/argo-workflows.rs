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
pub struct EventsourcePeriodEventSourceWatchEvent {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<Box<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodEventSource>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl EventsourcePeriodEventSourceWatchEvent {
    pub fn new() -> EventsourcePeriodEventSourceWatchEvent {
        EventsourcePeriodEventSourceWatchEvent {
            object: None,
            r#type: None,
        }
    }
}


