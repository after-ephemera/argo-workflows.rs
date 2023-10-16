# IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowLevelArtifactGc

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**force_finalizer_removal** | Option<**bool**> | ForceFinalizerRemoval: if set to true, the finalizer will be removed in the case that Artifact GC fails | [optional]
**pod_metadata** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMetadata**](io.argoproj.workflow.v1alpha1.Metadata.md)> |  | [optional]
**pod_spec_patch** | Option<**String**> | PodSpecPatch holds strategic merge patch to apply against the artgc pod spec. | [optional]
**service_account_name** | Option<**String**> | ServiceAccountName is an optional field for specifying the Service Account that should be assigned to the Pod doing the deletion | [optional]
**strategy** | Option<**String**> | Strategy is the strategy to use. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


