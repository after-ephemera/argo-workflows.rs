/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodOutputs : Outputs hold parameters, artifacts, and results from a step



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodOutputs {
    /// Artifacts holds the list of output artifacts produced by a step
    #[serde(rename = "artifacts", skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodArtifact>>,
    /// ExitCode holds the exit code of a script template
    #[serde(rename = "exitCode", skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<String>,
    /// Parameters holds the list of output parameters produced by a step
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodParameter>>,
    /// Result holds the result (stdout) of a script template
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodOutputs {
    /// Outputs hold parameters, artifacts, and results from a step
    pub fn new() -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodOutputs {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodOutputs {
            artifacts: None,
            exit_code: None,
            parameters: None,
            result: None,
        }
    }
}


