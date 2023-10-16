# IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodCronWorkflowStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | [**Vec<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodObjectReference>**](io.k8s.api.core.v1.ObjectReference.md) | Active is a list of active workflows stemming from this CronWorkflow | 
**conditions** | [**Vec<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodCondition>**](io.argoproj.workflow.v1alpha1.Condition.md) | Conditions is a list of conditions the CronWorkflow may have | 
**last_scheduled_time** | **String** | Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


