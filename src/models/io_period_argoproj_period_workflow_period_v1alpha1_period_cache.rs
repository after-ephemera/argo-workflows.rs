/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodCache : Cache is the configuration for the type of cache to be used



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodCache {
    #[serde(rename = "configMap")]
    pub config_map: Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodConfigMapKeySelector>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodCache {
    /// Cache is the configuration for the type of cache to be used
    pub fn new(config_map: crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodConfigMapKeySelector) -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodCache {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodCache {
            config_map: Box::new(config_map),
        }
    }
}


