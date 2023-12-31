/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodVolumeDevice : volumeDevice describes a mapping of a raw block device within a container.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodVolumeDevice {
    /// devicePath is the path inside of the container that the device will be mapped to.
    #[serde(rename = "devicePath")]
    pub device_path: String,
    /// name must match the name of a persistentVolumeClaim in the pod
    #[serde(rename = "name")]
    pub name: String,
}

impl IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodVolumeDevice {
    /// volumeDevice describes a mapping of a raw block device within a container.
    pub fn new(device_path: String, name: String) -> IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodVolumeDevice {
        IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodVolumeDevice {
            device_path,
            name,
        }
    }
}


