# IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodPrometheus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**counter** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodCounter**](io.argoproj.workflow.v1alpha1.Counter.md)> |  | [optional]
**gauge** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodGauge**](io.argoproj.workflow.v1alpha1.Gauge.md)> |  | [optional]
**help** | **String** | Help is a string that describes the metric | 
**histogram** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodHistogram**](io.argoproj.workflow.v1alpha1.Histogram.md)> |  | [optional]
**labels** | Option<[**Vec<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodMetricLabel>**](io.argoproj.workflow.v1alpha1.MetricLabel.md)> | Labels is a list of metric labels | [optional]
**name** | **String** | Name is the name of the metric | 
**when** | Option<**String**> | When is a conditional statement that decides when to emit the metric | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


