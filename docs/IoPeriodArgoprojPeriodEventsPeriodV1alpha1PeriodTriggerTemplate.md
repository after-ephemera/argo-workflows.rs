# IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTriggerTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**argo_workflow** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodArgoWorkflowTrigger**](io.argoproj.events.v1alpha1.ArgoWorkflowTrigger.md)> |  | [optional]
**aws_lambda** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodAwsLambdaTrigger**](io.argoproj.events.v1alpha1.AWSLambdaTrigger.md)> |  | [optional]
**azure_event_hubs** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodAzureEventHubsTrigger**](io.argoproj.events.v1alpha1.AzureEventHubsTrigger.md)> |  | [optional]
**conditions** | Option<**String**> |  | [optional]
**conditions_reset** | Option<[**Vec<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodConditionsResetCriteria>**](io.argoproj.events.v1alpha1.ConditionsResetCriteria.md)> |  | [optional]
**custom** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodCustomTrigger**](io.argoproj.events.v1alpha1.CustomTrigger.md)> |  | [optional]
**http** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodHttpTrigger**](io.argoproj.events.v1alpha1.HTTPTrigger.md)> |  | [optional]
**k8s** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodStandardK8STrigger**](io.argoproj.events.v1alpha1.StandardK8STrigger.md)> |  | [optional]
**kafka** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodKafkaTrigger**](io.argoproj.events.v1alpha1.KafkaTrigger.md)> |  | [optional]
**log** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodLogTrigger**](io.argoproj.events.v1alpha1.LogTrigger.md)> |  | [optional]
**name** | Option<**String**> | Name is a unique name of the action to take. | [optional]
**nats** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodNatsTrigger**](io.argoproj.events.v1alpha1.NATSTrigger.md)> |  | [optional]
**open_whisk** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodOpenWhiskTrigger**](io.argoproj.events.v1alpha1.OpenWhiskTrigger.md)> |  | [optional]
**pulsar** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodPulsarTrigger**](io.argoproj.events.v1alpha1.PulsarTrigger.md)> |  | [optional]
**slack** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSlackTrigger**](io.argoproj.events.v1alpha1.SlackTrigger.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


