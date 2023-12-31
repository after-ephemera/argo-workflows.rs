/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodExecAction : ExecAction describes a \"run in container\" action.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodExecAction {
    /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
    #[serde(rename = "command", skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
}

impl IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodExecAction {
    /// ExecAction describes a \"run in container\" action.
    pub fn new() -> IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodExecAction {
        IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodExecAction {
            command: None,
        }
    }
}


