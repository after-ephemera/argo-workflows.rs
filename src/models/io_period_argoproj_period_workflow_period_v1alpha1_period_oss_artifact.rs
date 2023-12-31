/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodOssArtifact : OSSArtifact is the location of an Alibaba Cloud OSS artifact



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodOssArtifact {
    #[serde(rename = "accessKeySecret", skip_serializing_if = "Option::is_none")]
    pub access_key_secret: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector>>,
    /// Bucket is the name of the bucket
    #[serde(rename = "bucket", skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// CreateBucketIfNotPresent tells the driver to attempt to create the OSS bucket for output artifacts, if it doesn't exist
    #[serde(rename = "createBucketIfNotPresent", skip_serializing_if = "Option::is_none")]
    pub create_bucket_if_not_present: Option<bool>,
    /// Endpoint is the hostname of the bucket endpoint
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// Key is the path in the bucket where the artifact resides
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "lifecycleRule", skip_serializing_if = "Option::is_none")]
    pub lifecycle_rule: Option<Box<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodOssLifecycleRule>>,
    #[serde(rename = "secretKeySecret", skip_serializing_if = "Option::is_none")]
    pub secret_key_secret: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector>>,
    /// SecurityToken is the user's temporary security token. For more details, check out: https://www.alibabacloud.com/help/doc-detail/100624.htm
    #[serde(rename = "securityToken", skip_serializing_if = "Option::is_none")]
    pub security_token: Option<String>,
    /// UseSDKCreds tells the driver to figure out credentials based on sdk defaults.
    #[serde(rename = "useSDKCreds", skip_serializing_if = "Option::is_none")]
    pub use_sdk_creds: Option<bool>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodOssArtifact {
    /// OSSArtifact is the location of an Alibaba Cloud OSS artifact
    pub fn new(key: String) -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodOssArtifact {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodOssArtifact {
            access_key_secret: None,
            bucket: None,
            create_bucket_if_not_present: None,
            endpoint: None,
            key,
            lifecycle_rule: None,
            secret_key_secret: None,
            security_token: None,
            use_sdk_creds: None,
        }
    }
}


