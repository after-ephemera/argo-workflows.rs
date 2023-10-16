/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodPodGc : PodGC describes how to delete completed pods as they complete



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodPodGc {
    #[serde(rename = "deleteDelayDuration", skip_serializing_if = "Option::is_none")]
    pub delete_delay_duration: Option<Box<crate::models::IoPeriodK8sPeriodApimachineryPeriodPkgPeriodApisPeriodMetaPeriodV1PeriodDuration>>,
    #[serde(rename = "labelSelector", skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<Box<crate::models::IoPeriodK8sPeriodApimachineryPeriodPkgPeriodApisPeriodMetaPeriodV1PeriodLabelSelector>>,
    /// Strategy is the strategy to use. One of \"OnPodCompletion\", \"OnPodSuccess\", \"OnWorkflowCompletion\", \"OnWorkflowSuccess\". If unset, does not delete Pods
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodPodGc {
    /// PodGC describes how to delete completed pods as they complete
    pub fn new() -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodPodGc {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodPodGc {
            delete_delay_duration: None,
            label_selector: None,
            strategy: None,
        }
    }
}

