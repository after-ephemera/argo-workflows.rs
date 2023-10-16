/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodAzureDiskVolumeSource : AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodAzureDiskVolumeSource {
    /// Host Caching mode: None, Read Only, Read Write.
    #[serde(rename = "cachingMode", skip_serializing_if = "Option::is_none")]
    pub caching_mode: Option<String>,
    /// The Name of the data disk in the blob storage
    #[serde(rename = "diskName")]
    pub disk_name: String,
    /// The URI the data disk in the blob storage
    #[serde(rename = "diskURI")]
    pub disk_uri: String,
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified.
    #[serde(rename = "fsType", skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    /// Expected values Shared: multiple blob disks per storage account  Dedicated: single blob disk per storage account  Managed: azure managed data disk (only in managed availability set). defaults to shared
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

impl IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodAzureDiskVolumeSource {
    /// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
    pub fn new(disk_name: String, disk_uri: String) -> IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodAzureDiskVolumeSource {
        IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodAzureDiskVolumeSource {
            caching_mode: None,
            disk_name,
            disk_uri,
            fs_type: None,
            kind: None,
            read_only: None,
        }
    }
}


