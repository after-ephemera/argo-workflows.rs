/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodClusterWorkflowTemplateList : ClusterWorkflowTemplateList is list of ClusterWorkflowTemplate resources



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodClusterWorkflowTemplateList {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(rename = "items")]
    pub items: Vec<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodClusterWorkflowTemplate>,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "metadata")]
    pub metadata: Box<crate::models::IoPeriodK8sPeriodApimachineryPeriodPkgPeriodApisPeriodMetaPeriodV1PeriodListMeta>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodClusterWorkflowTemplateList {
    /// ClusterWorkflowTemplateList is list of ClusterWorkflowTemplate resources
    pub fn new(items: Vec<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodClusterWorkflowTemplate>, metadata: crate::models::IoPeriodK8sPeriodApimachineryPeriodPkgPeriodApisPeriodMetaPeriodV1PeriodListMeta) -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodClusterWorkflowTemplateList {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodClusterWorkflowTemplateList {
            api_version: None,
            items,
            kind: None,
            metadata: Box::new(metadata),
        }
    }
}

