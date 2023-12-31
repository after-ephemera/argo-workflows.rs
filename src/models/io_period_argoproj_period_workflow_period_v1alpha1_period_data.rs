/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodData : Data is a data template



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodData {
    #[serde(rename = "source")]
    pub source: Box<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodDataSource>,
    /// Transformation applies a set of transformations
    #[serde(rename = "transformation")]
    pub transformation: Vec<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodTransformationStep>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodData {
    /// Data is a data template
    pub fn new(source: crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodDataSource, transformation: Vec<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodTransformationStep>) -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodData {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodData {
            source: Box::new(source),
            transformation,
        }
    }
}


