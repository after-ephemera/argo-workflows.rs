# IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodGitArtifact

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**branch** | Option<**String**> |  | [optional]
**clone_directory** | Option<**String**> | Directory to clone the repository. We clone complete directory because GitArtifact is not limited to any specific Git service providers. Hence we don't use any specific git provider client. | [optional]
**creds** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodGitCreds**](io.argoproj.events.v1alpha1.GitCreds.md)> |  | [optional]
**file_path** | Option<**String**> |  | [optional]
**insecure_ignore_host_key** | Option<**bool**> |  | [optional]
**r#ref** | Option<**String**> |  | [optional]
**remote** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodGitRemoteConfig**](io.argoproj.events.v1alpha1.GitRemoteConfig.md)> |  | [optional]
**ssh_key_secret** | Option<[**crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector**](io.k8s.api.core.v1.SecretKeySelector.md)> |  | [optional]
**tag** | Option<**String**> |  | [optional]
**url** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


