/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodEventSourceSpec {
    #[serde(rename = "amqp", skip_serializing_if = "Option::is_none")]
    pub amqp: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodAmqpEventSource>>,
    #[serde(rename = "azureEventsHub", skip_serializing_if = "Option::is_none")]
    pub azure_events_hub: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodAzureEventsHubEventSource>>,
    #[serde(rename = "bitbucket", skip_serializing_if = "Option::is_none")]
    pub bitbucket: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketEventSource>>,
    #[serde(rename = "bitbucketserver", skip_serializing_if = "Option::is_none")]
    pub bitbucketserver: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodBitbucketServerEventSource>>,
    #[serde(rename = "calendar", skip_serializing_if = "Option::is_none")]
    pub calendar: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodCalendarEventSource>>,
    #[serde(rename = "emitter", skip_serializing_if = "Option::is_none")]
    pub emitter: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodEmitterEventSource>>,
    #[serde(rename = "eventBusName", skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "file", skip_serializing_if = "Option::is_none")]
    pub file: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodFileEventSource>>,
    #[serde(rename = "generic", skip_serializing_if = "Option::is_none")]
    pub generic: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodGenericEventSource>>,
    #[serde(rename = "github", skip_serializing_if = "Option::is_none")]
    pub github: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodGithubEventSource>>,
    #[serde(rename = "gitlab", skip_serializing_if = "Option::is_none")]
    pub gitlab: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodGitlabEventSource>>,
    #[serde(rename = "hdfs", skip_serializing_if = "Option::is_none")]
    pub hdfs: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodHdfsEventSource>>,
    #[serde(rename = "kafka", skip_serializing_if = "Option::is_none")]
    pub kafka: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodKafkaEventSource>>,
    #[serde(rename = "minio", skip_serializing_if = "Option::is_none")]
    pub minio: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodS3Artifact>>,
    #[serde(rename = "mqtt", skip_serializing_if = "Option::is_none")]
    pub mqtt: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodMqttEventSource>>,
    #[serde(rename = "nats", skip_serializing_if = "Option::is_none")]
    pub nats: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodNatsEventsSource>>,
    #[serde(rename = "nsq", skip_serializing_if = "Option::is_none")]
    pub nsq: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodNsqEventSource>>,
    #[serde(rename = "pubSub", skip_serializing_if = "Option::is_none")]
    pub pub_sub: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodPubSubEventSource>>,
    #[serde(rename = "pulsar", skip_serializing_if = "Option::is_none")]
    pub pulsar: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodPulsarEventSource>>,
    #[serde(rename = "redis", skip_serializing_if = "Option::is_none")]
    pub redis: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodRedisEventSource>>,
    #[serde(rename = "redisStream", skip_serializing_if = "Option::is_none")]
    pub redis_stream: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodRedisStreamEventSource>>,
    #[serde(rename = "replicas", skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodResourceEventSource>>,
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<Box<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodService>>,
    #[serde(rename = "slack", skip_serializing_if = "Option::is_none")]
    pub slack: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSlackEventSource>>,
    #[serde(rename = "sns", skip_serializing_if = "Option::is_none")]
    pub sns: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSnsEventSource>>,
    #[serde(rename = "sqs", skip_serializing_if = "Option::is_none")]
    pub sqs: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodSqsEventSource>>,
    #[serde(rename = "storageGrid", skip_serializing_if = "Option::is_none")]
    pub storage_grid: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodStorageGridEventSource>>,
    #[serde(rename = "stripe", skip_serializing_if = "Option::is_none")]
    pub stripe: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodStripeEventSource>>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<Box<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTemplate>>,
    #[serde(rename = "webhook", skip_serializing_if = "Option::is_none")]
    pub webhook: Option<::std::collections::HashMap<String, crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodWebhookEventSource>>,
}

impl IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodEventSourceSpec {
    pub fn new() -> IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodEventSourceSpec {
        IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodEventSourceSpec {
            amqp: None,
            azure_events_hub: None,
            bitbucket: None,
            bitbucketserver: None,
            calendar: None,
            emitter: None,
            event_bus_name: None,
            file: None,
            generic: None,
            github: None,
            gitlab: None,
            hdfs: None,
            kafka: None,
            minio: None,
            mqtt: None,
            nats: None,
            nsq: None,
            pub_sub: None,
            pulsar: None,
            redis: None,
            redis_stream: None,
            replicas: None,
            resource: None,
            service: None,
            slack: None,
            sns: None,
            sqs: None,
            storage_grid: None,
            stripe: None,
            template: None,
            webhook: None,
        }
    }
}

