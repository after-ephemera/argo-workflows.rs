# IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketEventSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auth** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketAuth**](io.argoproj.events.v1alpha1.BitbucketAuth.md)> |  | [optional]
**delete_hook_on_finish** | Option<**bool**> |  | [optional]
**events** | Option<**Vec<String>**> | Events this webhook is subscribed to. | [optional]
**filter** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodEventSourceFilter**](io.argoproj.events.v1alpha1.EventSourceFilter.md)> |  | [optional]
**metadata** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**owner** | Option<**String**> |  | [optional]
**project_key** | Option<**String**> |  | [optional]
**repositories** | Option<[**Vec<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketRepository>**](io.argoproj.events.v1alpha1.BitbucketRepository.md)> |  | [optional]
**repository_slug** | Option<**String**> |  | [optional]
**webhook** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodWebhookContext**](io.argoproj.events.v1alpha1.WebhookContext.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


