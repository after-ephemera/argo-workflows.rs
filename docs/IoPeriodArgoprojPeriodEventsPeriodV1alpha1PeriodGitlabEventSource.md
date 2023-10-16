# IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodGitlabEventSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_token** | Option<[**crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector**](io.k8s.api.core.v1.SecretKeySelector.md)> |  | [optional]
**delete_hook_on_finish** | Option<**bool**> |  | [optional]
**enable_ssl_verification** | Option<**bool**> |  | [optional]
**events** | Option<**Vec<String>**> | Events are gitlab event to listen to. Refer https://github.com/xanzy/go-gitlab/blob/bf34eca5d13a9f4c3f501d8a97b8ac226d55e4d9/projects.go#L794. | [optional]
**filter** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodEventSourceFilter**](io.argoproj.events.v1alpha1.EventSourceFilter.md)> |  | [optional]
**gitlab_base_url** | Option<**String**> |  | [optional]
**metadata** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**project_id** | Option<**String**> |  | [optional]
**projects** | Option<**Vec<String>**> |  | [optional]
**secret_token** | Option<[**crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector**](io.k8s.api.core.v1.SecretKeySelector.md)> |  | [optional]
**webhook** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodWebhookContext**](io.argoproj.events.v1alpha1.WebhookContext.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


