/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodResourceRequirements : ResourceRequirements describes the compute resource requirements.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodResourceRequirements {
    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(rename = "limits", skip_serializing_if = "Option::is_none")]
    pub limits: Option<::std::collections::HashMap<String, String>>,
    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none")]
    pub requests: Option<::std::collections::HashMap<String, String>>,
}

impl IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodResourceRequirements {
    /// ResourceRequirements describes the compute resource requirements.
    pub fn new() -> IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodResourceRequirements {
        IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodResourceRequirements {
            limits: None,
            requests: None,
        }
    }
}


