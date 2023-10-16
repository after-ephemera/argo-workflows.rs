# IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_deadline_seconds** | Option<**String**> |  | [optional]
**affinity** | Option<[**crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodAffinity**](io.k8s.api.core.v1.Affinity.md)> |  | [optional]
**archive_location** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodArtifactLocation**](io.argoproj.workflow.v1alpha1.ArtifactLocation.md)> |  | [optional]
**automount_service_account_token** | Option<**bool**> | AutomountServiceAccountToken indicates whether a service account token should be automatically mounted in pods. ServiceAccountName of ExecutorConfig must be specified if this value is false. | [optional]
**container** | Option<[**crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodContainer**](io.k8s.api.core.v1.Container.md)> |  | [optional]
**container_set** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodContainerSetTemplate**](io.argoproj.workflow.v1alpha1.ContainerSetTemplate.md)> |  | [optional]
**daemon** | Option<**bool**> | Daemon will allow a workflow to proceed to the next step so long as the container reaches readiness | [optional]
**dag** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodDagTemplate**](io.argoproj.workflow.v1alpha1.DAGTemplate.md)> |  | [optional]
**data** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodData**](io.argoproj.workflow.v1alpha1.Data.md)> |  | [optional]
**executor** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodExecutorConfig**](io.argoproj.workflow.v1alpha1.ExecutorConfig.md)> |  | [optional]
**fail_fast** | Option<**bool**> | FailFast, if specified, will fail this template if any of its child pods has failed. This is useful for when this template is expanded with `withItems`, etc. | [optional]
**host_aliases** | Option<[**Vec<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodHostAlias>**](io.k8s.api.core.v1.HostAlias.md)> | HostAliases is an optional list of hosts and IPs that will be injected into the pod spec | [optional]
**http** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodHttp**](io.argoproj.workflow.v1alpha1.HTTP.md)> |  | [optional]
**init_containers** | Option<[**Vec<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodUserContainer>**](io.argoproj.workflow.v1alpha1.UserContainer.md)> | InitContainers is a list of containers which run before the main container. | [optional]
**inputs** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodInputs**](io.argoproj.workflow.v1alpha1.Inputs.md)> |  | [optional]
**memoize** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMemoize**](io.argoproj.workflow.v1alpha1.Memoize.md)> |  | [optional]
**metadata** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMetadata**](io.argoproj.workflow.v1alpha1.Metadata.md)> |  | [optional]
**metrics** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMetrics**](io.argoproj.workflow.v1alpha1.Metrics.md)> |  | [optional]
**name** | Option<**String**> | Name is the name of the template | [optional]
**node_selector** | Option<**::std::collections::HashMap<String, String>**> | NodeSelector is a selector to schedule this step of the workflow to be run on the selected node(s). Overrides the selector set at the workflow level. | [optional]
**outputs** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodOutputs**](io.argoproj.workflow.v1alpha1.Outputs.md)> |  | [optional]
**parallelism** | Option<**i32**> | Parallelism limits the max total parallel pods that can execute at the same time within the boundaries of this template invocation. If additional steps/dag templates are invoked, the pods created by those templates will not be counted towards this total. | [optional]
**plugin** | Option<[**serde_json::Value**](.md)> | Plugin is an Object with exactly one key | [optional]
**pod_spec_patch** | Option<**String**> | PodSpecPatch holds strategic merge patch to apply against the pod spec. Allows parameterization of container fields which are not strings (e.g. resource limits). | [optional]
**priority** | Option<**i32**> | Priority to apply to workflow pods. | [optional]
**priority_class_name** | Option<**String**> | PriorityClassName to apply to workflow pods. | [optional]
**resource** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodResourceTemplate**](io.argoproj.workflow.v1alpha1.ResourceTemplate.md)> |  | [optional]
**retry_strategy** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodRetryStrategy**](io.argoproj.workflow.v1alpha1.RetryStrategy.md)> |  | [optional]
**scheduler_name** | Option<**String**> | If specified, the pod will be dispatched by specified scheduler. Or it will be dispatched by workflow scope scheduler if specified. If neither specified, the pod will be dispatched by default scheduler. | [optional]
**script** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodScriptTemplate**](io.argoproj.workflow.v1alpha1.ScriptTemplate.md)> |  | [optional]
**security_context** | Option<[**crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodPodSecurityContext**](io.k8s.api.core.v1.PodSecurityContext.md)> |  | [optional]
**service_account_name** | Option<**String**> | ServiceAccountName to apply to workflow pods | [optional]
**sidecars** | Option<[**Vec<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodUserContainer>**](io.argoproj.workflow.v1alpha1.UserContainer.md)> | Sidecars is a list of containers which run alongside the main container Sidecars are automatically killed when the main container completes | [optional]
**steps** | Option<[**Vec<Vec<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodWorkflowStep>>**](array.md)> | Steps define a series of sequential/parallel workflow steps | [optional]
**suspend** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodSuspendTemplate**](io.argoproj.workflow.v1alpha1.SuspendTemplate.md)> |  | [optional]
**synchronization** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodSynchronization**](io.argoproj.workflow.v1alpha1.Synchronization.md)> |  | [optional]
**timeout** | Option<**String**> | Timeout allows to set the total node execution timeout duration counting from the node's start time. This duration also includes time in which the node spends in Pending state. This duration may not be applied to Step or DAG templates. | [optional]
**tolerations** | Option<[**Vec<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodToleration>**](io.k8s.api.core.v1.Toleration.md)> | Tolerations to apply to workflow pods. | [optional]
**volumes** | Option<[**Vec<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodVolume>**](io.k8s.api.core.v1.Volume.md)> | Volumes is a list of volumes that can be mounted by containers in a template. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


