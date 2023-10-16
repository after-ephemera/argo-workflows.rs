# IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodDagTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**arguments** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodArguments**](io.argoproj.workflow.v1alpha1.Arguments.md)> |  | [optional]
**continue_on** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodContinueOn**](io.argoproj.workflow.v1alpha1.ContinueOn.md)> |  | [optional]
**dependencies** | Option<**Vec<String>**> | Dependencies are name of other targets which this depends on | [optional]
**depends** | Option<**String**> | Depends are name of other targets which this depends on | [optional]
**hooks** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodLifecycleHook>**](io.argoproj.workflow.v1alpha1.LifecycleHook.md)> | Hooks hold the lifecycle hook which is invoked at lifecycle of task, irrespective of the success, failure, or error status of the primary task | [optional]
**inline** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodTemplate**](io.argoproj.workflow.v1alpha1.Template.md)> |  | [optional]
**name** | **String** | Name is the name of the target | 
**on_exit** | Option<**String**> | OnExit is a template reference which is invoked at the end of the template, irrespective of the success, failure, or error of the primary template. DEPRECATED: Use Hooks[exit].Template instead. | [optional]
**template** | Option<**String**> | Name of template to execute | [optional]
**template_ref** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodTemplateRef**](io.argoproj.workflow.v1alpha1.TemplateRef.md)> |  | [optional]
**when** | Option<**String**> | When is an expression in which the task should conditionally execute | [optional]
**with_items** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | WithItems expands a task into multiple parallel tasks from the items in the list | [optional]
**with_param** | Option<**String**> | WithParam expands a task into multiple parallel tasks from the value in the parameter, which is expected to be a JSON list. | [optional]
**with_sequence** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodSequence**](io.argoproj.workflow.v1alpha1.Sequence.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


