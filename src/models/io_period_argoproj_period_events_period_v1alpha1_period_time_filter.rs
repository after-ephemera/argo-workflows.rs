/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTimeFilter : TimeFilter describes a window in time. It filters out events that occur outside the time limits. In other words, only events that occur after Start and before Stop will pass this filter.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTimeFilter {
    /// Start is the beginning of a time window in UTC. Before this time, events for this dependency are ignored. Format is hh:mm:ss.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// Stop is the end of a time window in UTC. After or equal to this time, events for this dependency are ignored and Format is hh:mm:ss. If it is smaller than Start, it is treated as next day of Start (e.g.: 22:00:00-01:00:00 means 22:00:00-25:00:00).
    #[serde(rename = "stop", skip_serializing_if = "Option::is_none")]
    pub stop: Option<String>,
}

impl IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTimeFilter {
    /// TimeFilter describes a window in time. It filters out events that occur outside the time limits. In other words, only events that occur after Start and before Stop will pass this filter.
    pub fn new() -> IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTimeFilter {
        IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTimeFilter {
            start: None,
            stop: None,
        }
    }
}


