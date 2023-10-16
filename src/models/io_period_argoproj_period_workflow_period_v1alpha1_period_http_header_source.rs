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
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodHttpHeaderSource {
    #[serde(rename = "secretKeyRef", skip_serializing_if = "Option::is_none")]
    pub secret_key_ref: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector>>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodHttpHeaderSource {
    pub fn new() -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodHttpHeaderSource {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodHttpHeaderSource {
            secret_key_ref: None,
        }
    }
}

