/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodFcVolumeSource : Represents a Fibre Channel volume. Fibre Channel volumes can only be mounted as read/write once. Fibre Channel volumes support ownership management and SELinux relabeling.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodFcVolumeSource {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified.
    #[serde(rename = "fsType", skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    /// Optional: FC target lun number
    #[serde(rename = "lun", skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
    /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// Optional: FC target worldwide names (WWNs)
    #[serde(rename = "targetWWNs", skip_serializing_if = "Option::is_none")]
    pub target_wwns: Option<Vec<String>>,
    /// Optional: FC volume world wide identifiers (wwids) Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.
    #[serde(rename = "wwids", skip_serializing_if = "Option::is_none")]
    pub wwids: Option<Vec<String>>,
}

impl IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodFcVolumeSource {
    /// Represents a Fibre Channel volume. Fibre Channel volumes can only be mounted as read/write once. Fibre Channel volumes support ownership management and SELinux relabeling.
    pub fn new() -> IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodFcVolumeSource {
        IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodFcVolumeSource {
            fs_type: None,
            lun: None,
            read_only: None,
            target_wwns: None,
            wwids: None,
        }
    }
}


