/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodPersistentVolumeClaimStatus : PersistentVolumeClaimStatus is the current status of a persistent volume claim.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodPersistentVolumeClaimStatus {
    /// AccessModes contains the actual access modes the volume backing the PVC has. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
    #[serde(rename = "accessModes", skip_serializing_if = "Option::is_none")]
    pub access_modes: Option<Vec<String>>,
    /// The storage resource within AllocatedResources tracks the capacity allocated to a PVC. It may be larger than the actual capacity when a volume expansion operation is requested. For storage quota, the larger value from allocatedResources and PVC.spec.resources is used. If allocatedResources is not set, PVC.spec.resources alone is used for quota calculation. If a volume expansion capacity request is lowered, allocatedResources is only lowered if there are no expansion operations in progress and if the actual volume capacity is equal or lower than the requested capacity. This is an alpha field and requires enabling RecoverVolumeExpansionFailure feature.
    #[serde(rename = "allocatedResources", skip_serializing_if = "Option::is_none")]
    pub allocated_resources: Option<::std::collections::HashMap<String, String>>,
    /// Represents the actual resources of the underlying volume.
    #[serde(rename = "capacity", skip_serializing_if = "Option::is_none")]
    pub capacity: Option<::std::collections::HashMap<String, String>>,
    /// Current Condition of persistent volume claim. If underlying persistent volume is being resized then the Condition will be set to 'ResizeStarted'.
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodPersistentVolumeClaimCondition>>,
    /// Phase represents the current phase of PersistentVolumeClaim.  Possible enum values:  - `\"Bound\"` used for PersistentVolumeClaims that are bound  - `\"Lost\"` used for PersistentVolumeClaims that lost their underlying PersistentVolume. The claim was bound to a PersistentVolume and this volume does not exist any longer and all data on it was lost.  - `\"Pending\"` used for PersistentVolumeClaims that are not yet bound
    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<Phase>,
    /// ResizeStatus stores status of resize operation. ResizeStatus is not set by default but when expansion is complete resizeStatus is set to empty string by resize controller or kubelet. This is an alpha field and requires enabling RecoverVolumeExpansionFailure feature.
    #[serde(rename = "resizeStatus", skip_serializing_if = "Option::is_none")]
    pub resize_status: Option<String>,
}

impl IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodPersistentVolumeClaimStatus {
    /// PersistentVolumeClaimStatus is the current status of a persistent volume claim.
    pub fn new() -> IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodPersistentVolumeClaimStatus {
        IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodPersistentVolumeClaimStatus {
            access_modes: None,
            allocated_resources: None,
            capacity: None,
            conditions: None,
            phase: None,
            resize_status: None,
        }
    }
}

/// Phase represents the current phase of PersistentVolumeClaim.  Possible enum values:  - `\"Bound\"` used for PersistentVolumeClaims that are bound  - `\"Lost\"` used for PersistentVolumeClaims that lost their underlying PersistentVolume. The claim was bound to a PersistentVolume and this volume does not exist any longer and all data on it was lost.  - `\"Pending\"` used for PersistentVolumeClaims that are not yet bound
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Phase {
    #[serde(rename = "Bound")]
    Bound,
    #[serde(rename = "Lost")]
    Lost,
    #[serde(rename = "Pending")]
    Pending,
}

impl Default for Phase {
    fn default() -> Phase {
        Self::Bound
    }
}

