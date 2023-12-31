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
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodSubmit {
    #[serde(rename = "arguments", skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Box<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodArguments>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::IoPeriodK8sPeriodApimachineryPeriodPkgPeriodApisPeriodMetaPeriodV1PeriodObjectMeta>>,
    #[serde(rename = "workflowTemplateRef")]
    pub workflow_template_ref: Box<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowTemplateRef>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodSubmit {
    pub fn new(workflow_template_ref: crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowTemplateRef) -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodSubmit {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodSubmit {
            arguments: None,
            metadata: None,
            workflow_template_ref: Box::new(workflow_template_ref),
        }
    }
}


