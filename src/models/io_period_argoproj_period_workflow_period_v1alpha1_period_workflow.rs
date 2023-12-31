/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflow : Workflow is the definition of a workflow resource



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflow {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "metadata")]
    pub metadata: Box<crate::models::IoPeriodK8sPeriodApimachineryPeriodPkgPeriodApisPeriodMetaPeriodV1PeriodObjectMeta>,
    #[serde(rename = "spec")]
    pub spec: Box<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowSpec>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowStatus>>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflow {
    /// Workflow is the definition of a workflow resource
    pub fn new(metadata: crate::models::IoPeriodK8sPeriodApimachineryPeriodPkgPeriodApisPeriodMetaPeriodV1PeriodObjectMeta, spec: crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowSpec) -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflow {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflow {
            api_version: None,
            kind: None,
            metadata: Box::new(metadata),
            spec: Box::new(spec),
            status: None,
        }
    }
}


