/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodDagTemplate : DAGTemplate is a template subtype for directed acyclic graph templates



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodDagTemplate {
    /// This flag is for DAG logic. The DAG logic has a built-in \"fail fast\" feature to stop scheduling new steps, as soon as it detects that one of the DAG nodes is failed. Then it waits until all DAG nodes are completed before failing the DAG itself. The FailFast flag default is true,  if set to false, it will allow a DAG to run all branches of the DAG to completion (either success or failure), regardless of the failed outcomes of branches in the DAG. More info and example about this feature at https://github.com/argoproj/argo-workflows/issues/1442
    #[serde(rename = "failFast", skip_serializing_if = "Option::is_none")]
    pub fail_fast: Option<bool>,
    /// Target are one or more names of targets to execute in a DAG
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// Tasks are a list of DAG tasks
    #[serde(rename = "tasks")]
    pub tasks: Vec<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodDagTask>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodDagTemplate {
    /// DAGTemplate is a template subtype for directed acyclic graph templates
    pub fn new(tasks: Vec<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodDagTask>) -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodDagTemplate {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodDagTemplate {
            fail_fast: None,
            target: None,
            tasks,
        }
    }
}


