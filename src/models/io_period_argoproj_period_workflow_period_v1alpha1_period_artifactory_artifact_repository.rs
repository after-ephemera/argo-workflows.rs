/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodArtifactoryArtifactRepository : ArtifactoryArtifactRepository defines the controller configuration for an artifactory artifact repository



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodArtifactoryArtifactRepository {
    /// KeyFormat defines the format of how to store keys and can reference workflow variables.
    #[serde(rename = "keyFormat", skip_serializing_if = "Option::is_none")]
    pub key_format: Option<String>,
    #[serde(rename = "passwordSecret", skip_serializing_if = "Option::is_none")]
    pub password_secret: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector>>,
    /// RepoURL is the url for artifactory repo.
    #[serde(rename = "repoURL", skip_serializing_if = "Option::is_none")]
    pub repo_url: Option<String>,
    #[serde(rename = "usernameSecret", skip_serializing_if = "Option::is_none")]
    pub username_secret: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector>>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodArtifactoryArtifactRepository {
    /// ArtifactoryArtifactRepository defines the controller configuration for an artifactory artifact repository
    pub fn new() -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodArtifactoryArtifactRepository {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodArtifactoryArtifactRepository {
            key_format: None,
            password_secret: None,
            repo_url: None,
            username_secret: None,
        }
    }
}

