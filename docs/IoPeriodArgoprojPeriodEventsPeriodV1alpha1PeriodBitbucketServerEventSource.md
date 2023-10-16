# IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketServerEventSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_token** | Option<[**crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector**](io.k8s.api.core.v1.SecretKeySelector.md)> |  | [optional]
**bitbucketserver_base_url** | Option<**String**> |  | [optional]
**delete_hook_on_finish** | Option<**bool**> |  | [optional]
**events** | Option<**Vec<String>**> |  | [optional]
**filter** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodEventSourceFilter**](io.argoproj.events.v1alpha1.EventSourceFilter.md)> |  | [optional]
**metadata** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**project_key** | Option<**String**> |  | [optional]
**repositories** | Option<[**Vec<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketServerRepository>**](io.argoproj.events.v1alpha1.BitbucketServerRepository.md)> |  | [optional]
**repository_slug** | Option<**String**> |  | [optional]
**webhook** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodWebhookContext**](io.argoproj.events.v1alpha1.WebhookContext.md)> |  | [optional]
**webhook_secret** | Option<[**crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector**](io.k8s.api.core.v1.SecretKeySelector.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


