# IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodPersistentVolumeClaimSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_modes** | Option<**Vec<String>**> | AccessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1 | [optional]
**data_source** | Option<[**crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodTypedLocalObjectReference**](io.k8s.api.core.v1.TypedLocalObjectReference.md)> |  | [optional]
**data_source_ref** | Option<[**crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodTypedLocalObjectReference**](io.k8s.api.core.v1.TypedLocalObjectReference.md)> |  | [optional]
**resources** | Option<[**crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodResourceRequirements**](io.k8s.api.core.v1.ResourceRequirements.md)> |  | [optional]
**selector** | Option<[**crate::models::IoPeriodK8sPeriodApimachineryPeriodPkgPeriodApisPeriodMetaPeriodV1PeriodLabelSelector**](io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector.md)> |  | [optional]
**storage_class_name** | Option<**String**> | Name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1 | [optional]
**volume_mode** | Option<**String**> | volumeMode defines what type of volume is required by the claim. Value of Filesystem is implied when not included in claim spec. | [optional]
**volume_name** | Option<**String**> | VolumeName is the binding reference to the PersistentVolume backing this claim. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


