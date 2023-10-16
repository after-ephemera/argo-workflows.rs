/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMutex : Mutex holds Mutex configuration



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMutex {
    /// name of the mutex
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace is the namespace of the mutex, default: [namespace of workflow]
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMutex {
    /// Mutex holds Mutex configuration
    pub fn new() -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMutex {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMutex {
            name: None,
            namespace: None,
        }
    }
}


