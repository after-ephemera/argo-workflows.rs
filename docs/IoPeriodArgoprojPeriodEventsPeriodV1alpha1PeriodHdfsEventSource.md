# IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodHdfsEventSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**addresses** | Option<**Vec<String>**> |  | [optional]
**check_interval** | Option<**String**> |  | [optional]
**filter** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodEventSourceFilter**](io.argoproj.events.v1alpha1.EventSourceFilter.md)> |  | [optional]
**hdfs_user** | Option<**String**> | HDFSUser is the user to access HDFS file system. It is ignored if either ccache or keytab is used. | [optional]
**krb_c_cache_secret** | Option<[**crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector**](io.k8s.api.core.v1.SecretKeySelector.md)> |  | [optional]
**krb_config_config_map** | Option<[**crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodConfigMapKeySelector**](io.k8s.api.core.v1.ConfigMapKeySelector.md)> |  | [optional]
**krb_keytab_secret** | Option<[**crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodSecretKeySelector**](io.k8s.api.core.v1.SecretKeySelector.md)> |  | [optional]
**krb_realm** | Option<**String**> | KrbRealm is the Kerberos realm used with Kerberos keytab It must be set if keytab is used. | [optional]
**krb_service_principal_name** | Option<**String**> | KrbServicePrincipalName is the principal name of Kerberos service It must be set if either ccache or keytab is used. | [optional]
**krb_username** | Option<**String**> | KrbUsername is the Kerberos username used with Kerberos keytab It must be set if keytab is used. | [optional]
**metadata** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**r#type** | Option<**String**> |  | [optional]
**watch_path_config** | Option<[**crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodWatchPathConfig**](io.argoproj.events.v1alpha1.WatchPathConfig.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


