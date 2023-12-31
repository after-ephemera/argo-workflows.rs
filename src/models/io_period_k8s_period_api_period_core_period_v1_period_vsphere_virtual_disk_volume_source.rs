/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodVsphereVirtualDiskVolumeSource : Represents a vSphere volume resource.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodVsphereVirtualDiskVolumeSource {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified.
    #[serde(rename = "fsType", skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    /// Storage Policy Based Management (SPBM) profile ID associated with the StoragePolicyName.
    #[serde(rename = "storagePolicyID", skip_serializing_if = "Option::is_none")]
    pub storage_policy_id: Option<String>,
    /// Storage Policy Based Management (SPBM) profile name.
    #[serde(rename = "storagePolicyName", skip_serializing_if = "Option::is_none")]
    pub storage_policy_name: Option<String>,
    /// Path that identifies vSphere volume vmdk
    #[serde(rename = "volumePath")]
    pub volume_path: String,
}

impl IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodVsphereVirtualDiskVolumeSource {
    /// Represents a vSphere volume resource.
    pub fn new(volume_path: String) -> IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodVsphereVirtualDiskVolumeSource {
        IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodVsphereVirtualDiskVolumeSource {
            fs_type: None,
            storage_policy_id: None,
            storage_policy_name: None,
            volume_path,
        }
    }
}


