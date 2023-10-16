/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodS3ArtifactRepository : S3ArtifactRepository defines the controller configuration for an S3 artifact repository



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodS3ArtifactRepository {
    #[serde(rename = "accessKeySecret", skip_serializing_if = "Option::is_none")]
    pub access_key_secret: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector>>,
    /// Bucket is the name of the bucket
    #[serde(rename = "bucket", skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "caSecret", skip_serializing_if = "Option::is_none")]
    pub ca_secret: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector>>,
    #[serde(rename = "createBucketIfNotPresent", skip_serializing_if = "Option::is_none")]
    pub create_bucket_if_not_present: Option<Box<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodCreateS3BucketOptions>>,
    #[serde(rename = "encryptionOptions", skip_serializing_if = "Option::is_none")]
    pub encryption_options: Option<Box<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodS3EncryptionOptions>>,
    /// Endpoint is the hostname of the bucket endpoint
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// Insecure will connect to the service with TLS
    #[serde(rename = "insecure", skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    /// KeyFormat defines the format of how to store keys and can reference workflow variables.
    #[serde(rename = "keyFormat", skip_serializing_if = "Option::is_none")]
    pub key_format: Option<String>,
    /// KeyPrefix is prefix used as part of the bucket key in which the controller will store artifacts. DEPRECATED. Use KeyFormat instead
    #[serde(rename = "keyPrefix", skip_serializing_if = "Option::is_none")]
    pub key_prefix: Option<String>,
    /// Region contains the optional bucket region
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// RoleARN is the Amazon Resource Name (ARN) of the role to assume.
    #[serde(rename = "roleARN", skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "secretKeySecret", skip_serializing_if = "Option::is_none")]
    pub secret_key_secret: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector>>,
    /// UseSDKCreds tells the driver to figure out credentials based on sdk defaults.
    #[serde(rename = "useSDKCreds", skip_serializing_if = "Option::is_none")]
    pub use_sdk_creds: Option<bool>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodS3ArtifactRepository {
    /// S3ArtifactRepository defines the controller configuration for an S3 artifact repository
    pub fn new() -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodS3ArtifactRepository {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodS3ArtifactRepository {
            access_key_secret: None,
            bucket: None,
            ca_secret: None,
            create_bucket_if_not_present: None,
            encryption_options: None,
            endpoint: None,
            insecure: None,
            key_format: None,
            key_prefix: None,
            region: None,
            role_arn: None,
            secret_key_secret: None,
            use_sdk_creds: None,
        }
    }
}


