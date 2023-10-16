# IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**artifact_gc_status** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodArtGcStatus**](io.argoproj.workflow.v1alpha1.ArtGCStatus.md)> |  | [optional]
**artifact_repository_ref** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodArtifactRepositoryRefStatus**](io.argoproj.workflow.v1alpha1.ArtifactRepositoryRefStatus.md)> |  | [optional]
**compressed_nodes** | Option<**String**> | Compressed and base64 decoded Nodes map | [optional]
**conditions** | Option<[**Vec<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodCondition>**](io.argoproj.workflow.v1alpha1.Condition.md)> | Conditions is a list of conditions the Workflow may have | [optional]
**estimated_duration** | Option<**i32**> | EstimatedDuration in seconds. | [optional]
**finished_at** | Option<**String**> | Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers. | [optional]
**message** | Option<**String**> | A human readable message indicating details about why the workflow is in this condition. | [optional]
**nodes** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodNodeStatus>**](io.argoproj.workflow.v1alpha1.NodeStatus.md)> | Nodes is a mapping between a node ID and the node's status. | [optional]
**offload_node_status_version** | Option<**String**> | Whether on not node status has been offloaded to a database. If exists, then Nodes and CompressedNodes will be empty. This will actually be populated with a hash of the offloaded data. | [optional]
**outputs** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodOutputs**](io.argoproj.workflow.v1alpha1.Outputs.md)> |  | [optional]
**persistent_volume_claims** | Option<[**Vec<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodVolume>**](io.k8s.api.core.v1.Volume.md)> | PersistentVolumeClaims tracks all PVCs that were created as part of the io.argoproj.workflow.v1alpha1. The contents of this list are drained at the end of the workflow. | [optional]
**phase** | Option<**String**> | Phase a simple, high-level summary of where the workflow is in its lifecycle. Will be \"\" (Unknown), \"Pending\", or \"Running\" before the workflow is completed, and \"Succeeded\", \"Failed\" or \"Error\" once the workflow has completed. | [optional]
**progress** | Option<**String**> | Progress to completion | [optional]
**resources_duration** | Option<**::std::collections::HashMap<String, i64>**> | ResourcesDuration is the total for the workflow | [optional]
**started_at** | Option<**String**> | Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers. | [optional]
**stored_templates** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodTemplate>**](io.argoproj.workflow.v1alpha1.Template.md)> | StoredTemplates is a mapping between a template ref and the node's status. | [optional]
**stored_workflow_template_spec** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowSpec**](io.argoproj.workflow.v1alpha1.WorkflowSpec.md)> |  | [optional]
**synchronization** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodSynchronizationStatus**](io.argoproj.workflow.v1alpha1.SynchronizationStatus.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


