# IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodLifecycleHook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**arguments** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodArguments**](io.argoproj.workflow.v1alpha1.Arguments.md)> |  | [optional]
**expression** | Option<**String**> | Expression is a condition expression for when a node will be retried. If it evaluates to false, the node will not be retried and the retry strategy will be ignored | [optional]
**template** | Option<**String**> | Template is the name of the template to execute by the hook | [optional]
**template_ref** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodTemplateRef**](io.argoproj.workflow.v1alpha1.TemplateRef.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


