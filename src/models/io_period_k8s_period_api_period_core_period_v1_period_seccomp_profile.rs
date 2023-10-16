/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSeccompProfile : SeccompProfile defines a pod/container's seccomp profile settings. Only one profile source may be set.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSeccompProfile {
    /// localhostProfile indicates a profile defined in a file on the node should be used. The profile must be preconfigured on the node to work. Must be a descending path, relative to the kubelet's configured seccomp profile location. Must only be set if type is \"Localhost\".
    #[serde(rename = "localhostProfile", skip_serializing_if = "Option::is_none")]
    pub localhost_profile: Option<String>,
    /// type indicates which kind of seccomp profile will be applied. Valid options are:  Localhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.  Possible enum values:  - `\"Localhost\"` indicates a profile defined in a file on the node should be used. The file's location relative to <kubelet-root-dir>/seccomp.  - `\"RuntimeDefault\"` represents the default container runtime seccomp profile.  - `\"Unconfined\"` indicates no seccomp profile is applied (A.K.A. unconfined).
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSeccompProfile {
    /// SeccompProfile defines a pod/container's seccomp profile settings. Only one profile source may be set.
    pub fn new(r#type: Type) -> IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSeccompProfile {
        IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSeccompProfile {
            localhost_profile: None,
            r#type,
        }
    }
}

/// type indicates which kind of seccomp profile will be applied. Valid options are:  Localhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.  Possible enum values:  - `\"Localhost\"` indicates a profile defined in a file on the node should be used. The file's location relative to <kubelet-root-dir>/seccomp.  - `\"RuntimeDefault\"` represents the default container runtime seccomp profile.  - `\"Unconfined\"` indicates no seccomp profile is applied (A.K.A. unconfined).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Localhost")]
    Localhost,
    #[serde(rename = "RuntimeDefault")]
    RuntimeDefault,
    #[serde(rename = "Unconfined")]
    Unconfined,
}

impl Default for Type {
    fn default() -> Type {
        Self::Localhost
    }
}

