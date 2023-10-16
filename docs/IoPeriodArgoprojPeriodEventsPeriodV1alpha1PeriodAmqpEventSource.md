# IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodAmqpEventSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auth** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBasicAuth**](io.argoproj.events.v1alpha1.BasicAuth.md)> |  | [optional]
**connection_backoff** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBackoff**](io.argoproj.events.v1alpha1.Backoff.md)> |  | [optional]
**consume** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodAmqpConsumeConfig**](io.argoproj.events.v1alpha1.AMQPConsumeConfig.md)> |  | [optional]
**exchange_declare** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodAmqpExchangeDeclareConfig**](io.argoproj.events.v1alpha1.AMQPExchangeDeclareConfig.md)> |  | [optional]
**exchange_name** | Option<**String**> |  | [optional]
**exchange_type** | Option<**String**> |  | [optional]
**filter** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodEventSourceFilter**](io.argoproj.events.v1alpha1.EventSourceFilter.md)> |  | [optional]
**json_body** | Option<**bool**> |  | [optional]
**metadata** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**queue_bind** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodAmqpQueueBindConfig**](io.argoproj.events.v1alpha1.AMQPQueueBindConfig.md)> |  | [optional]
**queue_declare** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodAmqpQueueDeclareConfig**](io.argoproj.events.v1alpha1.AMQPQueueDeclareConfig.md)> |  | [optional]
**routing_key** | Option<**String**> |  | [optional]
**tls** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTlsConfig**](io.argoproj.events.v1alpha1.TLSConfig.md)> |  | [optional]
**url** | Option<**String**> |  | [optional]
**url_secret** | Option<[**crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector**](io.k8s.api.core.v1.SecretKeySelector.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


