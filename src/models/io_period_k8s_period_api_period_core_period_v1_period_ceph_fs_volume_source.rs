/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodCephFsVolumeSource : Represents a Ceph Filesystem mount that lasts the lifetime of a pod Cephfs volumes do not support ownership management or SELinux relabeling.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodCephFsVolumeSource {
    /// Required: Monitors is a collection of Ceph monitors More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    #[serde(rename = "monitors")]
    pub monitors: Vec<String>,
    /// Optional: Used as the mounted root, rather than the full Ceph tree, default is /
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    #[serde(rename = "secretFile", skip_serializing_if = "Option::is_none")]
    pub secret_file: Option<String>,
    #[serde(rename = "secretRef", skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodLocalObjectReference>>,
    /// Optional: User is the rados user name, default is admin More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodCephFsVolumeSource {
    /// Represents a Ceph Filesystem mount that lasts the lifetime of a pod Cephfs volumes do not support ownership management or SELinux relabeling.
    pub fn new(monitors: Vec<String>) -> IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodCephFsVolumeSource {
        IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodCephFsVolumeSource {
            monitors,
            path: None,
            read_only: None,
            secret_file: None,
            secret_ref: None,
            user: None,
        }
    }
}


