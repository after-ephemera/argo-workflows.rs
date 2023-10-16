/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodArtifactLocation : ArtifactLocation describes a location for a single or multiple artifacts. It is used as single artifact in the context of inputs/outputs (e.g. outputs.artifacts.artname). It is also used to describe the location of multiple artifacts such as the archive location of a single workflow step, which the executor will use as a default location to store its files.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodArtifactLocation {
    /// ArchiveLogs indicates if the container logs should be archived
    #[serde(rename = "archiveLogs", skip_serializing_if = "Option::is_none")]
    pub archive_logs: Option<bool>,
    #[serde(rename = "artifactory", skip_serializing_if = "Option::is_none")]
    pub artifactory: Option<Box<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodArtifactoryArtifact>>,
    #[serde(rename = "azure", skip_serializing_if = "Option::is_none")]
    pub azure: Option<Box<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodAzureArtifact>>,
    #[serde(rename = "gcs", skip_serializing_if = "Option::is_none")]
    pub gcs: Option<Box<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodGcsArtifact>>,
    #[serde(rename = "git", skip_serializing_if = "Option::is_none")]
    pub git: Option<Box<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodGitArtifact>>,
    #[serde(rename = "hdfs", skip_serializing_if = "Option::is_none")]
    pub hdfs: Option<Box<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodHdfsArtifact>>,
    #[serde(rename = "http", skip_serializing_if = "Option::is_none")]
    pub http: Option<Box<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodHttpArtifact>>,
    #[serde(rename = "oss", skip_serializing_if = "Option::is_none")]
    pub oss: Option<Box<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodOssArtifact>>,
    #[serde(rename = "raw", skip_serializing_if = "Option::is_none")]
    pub raw: Option<Box<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodRawArtifact>>,
    #[serde(rename = "s3", skip_serializing_if = "Option::is_none")]
    pub s3: Option<Box<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodS3Artifact>>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodArtifactLocation {
    /// ArtifactLocation describes a location for a single or multiple artifacts. It is used as single artifact in the context of inputs/outputs (e.g. outputs.artifacts.artname). It is also used to describe the location of multiple artifacts such as the archive location of a single workflow step, which the executor will use as a default location to store its files.
    pub fn new() -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodArtifactLocation {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodArtifactLocation {
            archive_logs: None,
            artifactory: None,
            azure: None,
            gcs: None,
            git: None,
            hdfs: None,
            http: None,
            oss: None,
            raw: None,
            s3: None,
        }
    }
}


