/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodArtifactoryArtifact : ArtifactoryArtifact is the location of an artifactory artifact



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodArtifactoryArtifact {
    #[serde(rename = "passwordSecret", skip_serializing_if = "Option::is_none")]
    pub password_secret: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector>>,
    /// URL of the artifact
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "usernameSecret", skip_serializing_if = "Option::is_none")]
    pub username_secret: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector>>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodArtifactoryArtifact {
    /// ArtifactoryArtifact is the location of an artifactory artifact
    pub fn new(url: String) -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodArtifactoryArtifact {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodArtifactoryArtifact {
            password_secret: None,
            url,
            username_secret: None,
        }
    }
}

