# IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodStorageGridEventSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_url** | Option<**String**> | APIURL is the url of the storagegrid api. | [optional]
**auth_token** | Option<[**crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector**](io.k8s.api.core.v1.SecretKeySelector.md)> |  | [optional]
**bucket** | Option<**String**> | Name of the bucket to register notifications for. | [optional]
**events** | Option<**Vec<String>**> |  | [optional]
**filter** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodStorageGridFilter**](io.argoproj.events.v1alpha1.StorageGridFilter.md)> |  | [optional]
**metadata** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**region** | Option<**String**> |  | [optional]
**topic_arn** | Option<**String**> |  | [optional]
**webhook** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodWebhookContext**](io.argoproj.events.v1alpha1.WebhookContext.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


