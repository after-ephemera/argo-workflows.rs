# IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodPulsarEventSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auth_token_secret** | Option<[**crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector**](io.k8s.api.core.v1.SecretKeySelector.md)> |  | [optional]
**connection_backoff** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBackoff**](io.argoproj.events.v1alpha1.Backoff.md)> |  | [optional]
**filter** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodEventSourceFilter**](io.argoproj.events.v1alpha1.EventSourceFilter.md)> |  | [optional]
**json_body** | Option<**bool**> |  | [optional]
**metadata** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**tls** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTlsConfig**](io.argoproj.events.v1alpha1.TLSConfig.md)> |  | [optional]
**tls_allow_insecure_connection** | Option<**bool**> |  | [optional]
**tls_trust_certs_secret** | Option<[**crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector**](io.k8s.api.core.v1.SecretKeySelector.md)> |  | [optional]
**tls_validate_hostname** | Option<**bool**> |  | [optional]
**topics** | Option<**Vec<String>**> |  | [optional]
**r#type** | Option<**String**> |  | [optional]
**url** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


