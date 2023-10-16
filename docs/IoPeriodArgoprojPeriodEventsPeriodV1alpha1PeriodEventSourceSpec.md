# IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodEventSourceSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amqp** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodAmqpEventSource>**](io.argoproj.events.v1alpha1.AMQPEventSource.md)> |  | [optional]
**azure_events_hub** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodAzureEventsHubEventSource>**](io.argoproj.events.v1alpha1.AzureEventsHubEventSource.md)> |  | [optional]
**bitbucket** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketEventSource>**](io.argoproj.events.v1alpha1.BitbucketEventSource.md)> |  | [optional]
**bitbucketserver** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketServerEventSource>**](io.argoproj.events.v1alpha1.BitbucketServerEventSource.md)> |  | [optional]
**calendar** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodCalendarEventSource>**](io.argoproj.events.v1alpha1.CalendarEventSource.md)> |  | [optional]
**emitter** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodEmitterEventSource>**](io.argoproj.events.v1alpha1.EmitterEventSource.md)> |  | [optional]
**event_bus_name** | Option<**String**> |  | [optional]
**file** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodFileEventSource>**](io.argoproj.events.v1alpha1.FileEventSource.md)> |  | [optional]
**generic** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodGenericEventSource>**](io.argoproj.events.v1alpha1.GenericEventSource.md)> |  | [optional]
**github** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodGithubEventSource>**](io.argoproj.events.v1alpha1.GithubEventSource.md)> |  | [optional]
**gitlab** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodGitlabEventSource>**](io.argoproj.events.v1alpha1.GitlabEventSource.md)> |  | [optional]
**hdfs** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodHdfsEventSource>**](io.argoproj.events.v1alpha1.HDFSEventSource.md)> |  | [optional]
**kafka** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodKafkaEventSource>**](io.argoproj.events.v1alpha1.KafkaEventSource.md)> |  | [optional]
**minio** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodS3Artifact>**](io.argoproj.events.v1alpha1.S3Artifact.md)> |  | [optional]
**mqtt** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodMqttEventSource>**](io.argoproj.events.v1alpha1.MQTTEventSource.md)> |  | [optional]
**nats** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodNatsEventsSource>**](io.argoproj.events.v1alpha1.NATSEventsSource.md)> |  | [optional]
**nsq** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodNsqEventSource>**](io.argoproj.events.v1alpha1.NSQEventSource.md)> |  | [optional]
**pub_sub** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodPubSubEventSource>**](io.argoproj.events.v1alpha1.PubSubEventSource.md)> |  | [optional]
**pulsar** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodPulsarEventSource>**](io.argoproj.events.v1alpha1.PulsarEventSource.md)> |  | [optional]
**redis** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodRedisEventSource>**](io.argoproj.events.v1alpha1.RedisEventSource.md)> |  | [optional]
**redis_stream** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodRedisStreamEventSource>**](io.argoproj.events.v1alpha1.RedisStreamEventSource.md)> |  | [optional]
**replicas** | Option<**i32**> |  | [optional]
**resource** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodResourceEventSource>**](io.argoproj.events.v1alpha1.ResourceEventSource.md)> |  | [optional]
**service** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodService**](io.argoproj.events.v1alpha1.Service.md)> |  | [optional]
**slack** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSlackEventSource>**](io.argoproj.events.v1alpha1.SlackEventSource.md)> |  | [optional]
**sns** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSnsEventSource>**](io.argoproj.events.v1alpha1.SNSEventSource.md)> |  | [optional]
**sqs** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSqsEventSource>**](io.argoproj.events.v1alpha1.SQSEventSource.md)> |  | [optional]
**storage_grid** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodStorageGridEventSource>**](io.argoproj.events.v1alpha1.StorageGridEventSource.md)> |  | [optional]
**stripe** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodStripeEventSource>**](io.argoproj.events.v1alpha1.StripeEventSource.md)> |  | [optional]
**template** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTemplate**](io.argoproj.events.v1alpha1.Template.md)> |  | [optional]
**webhook** | Option<[**::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodWebhookEventSource>**](io.argoproj.events.v1alpha1.WebhookEventSource.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


