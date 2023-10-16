# IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodOssArtifactRepository

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_key_secret** | Option<[**crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector**](io.k8s.api.core.v1.SecretKeySelector.md)> |  | [optional]
**bucket** | Option<**String**> | Bucket is the name of the bucket | [optional]
**create_bucket_if_not_present** | Option<**bool**> | CreateBucketIfNotPresent tells the driver to attempt to create the OSS bucket for output artifacts, if it doesn't exist | [optional]
**endpoint** | Option<**String**> | Endpoint is the hostname of the bucket endpoint | [optional]
**key_format** | Option<**String**> | KeyFormat defines the format of how to store keys and can reference workflow variables. | [optional]
**lifecycle_rule** | Option<[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodOssLifecycleRule**](io.argoproj.workflow.v1alpha1.OSSLifecycleRule.md)> |  | [optional]
**secret_key_secret** | Option<[**crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector**](io.k8s.api.core.v1.SecretKeySelector.md)> |  | [optional]
**security_token** | Option<**String**> | SecurityToken is the user's temporary security token. For more details, check out: https://www.alibabacloud.com/help/doc-detail/100624.htm | [optional]
**use_sdk_creds** | Option<**bool**> | UseSDKCreds tells the driver to figure out credentials based on sdk defaults. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


