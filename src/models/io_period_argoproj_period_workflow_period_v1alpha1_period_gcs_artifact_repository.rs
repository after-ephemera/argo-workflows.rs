/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodGcsArtifactRepository : GCSArtifactRepository defines the controller configuration for a GCS artifact repository



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodGcsArtifactRepository {
    /// Bucket is the name of the bucket
    #[serde(rename = "bucket", skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// KeyFormat defines the format of how to store keys and can reference workflow variables.
    #[serde(rename = "keyFormat", skip_serializing_if = "Option::is_none")]
    pub key_format: Option<String>,
    #[serde(rename = "serviceAccountKeySecret", skip_serializing_if = "Option::is_none")]
    pub service_account_key_secret: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector>>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodGcsArtifactRepository {
    /// GCSArtifactRepository defines the controller configuration for a GCS artifact repository
    pub fn new() -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodGcsArtifactRepository {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodGcsArtifactRepository {
            bucket: None,
            key_format: None,
            service_account_key_secret: None,
        }
    }
}


