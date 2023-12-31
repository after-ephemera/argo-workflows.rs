# IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodDagTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fail_fast** | Option<**bool**> | This flag is for DAG logic. The DAG logic has a built-in \"fail fast\" feature to stop scheduling new steps, as soon as it detects that one of the DAG nodes is failed. Then it waits until all DAG nodes are completed before failing the DAG itself. The FailFast flag default is true,  if set to false, it will allow a DAG to run all branches of the DAG to completion (either success or failure), regardless of the failed outcomes of branches in the DAG. More info and example about this feature at https://github.com/argoproj/argo-workflows/issues/1442 | [optional]
**target** | Option<**String**> | Target are one or more names of targets to execute in a DAG | [optional]
**tasks** | [**Vec<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodDagTask>**](io.argoproj.workflow.v1alpha1.DAGTask.md) | Tasks are a list of DAG tasks | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


