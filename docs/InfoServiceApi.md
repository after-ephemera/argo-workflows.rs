# \InfoServiceApi

All URIs are relative to *http://localhost:2746*

Method | HTTP request | Description
------------- | ------------- | -------------
[**info_service_collect_event**](InfoServiceApi.md#info_service_collect_event) | **POST** /api/v1/tracking/event | 
[**info_service_get_info**](InfoServiceApi.md#info_service_get_info) | **GET** /api/v1/info | 
[**info_service_get_user_info**](InfoServiceApi.md#info_service_get_user_info) | **GET** /api/v1/userinfo | 
[**info_service_get_version**](InfoServiceApi.md#info_service_get_version) | **GET** /api/v1/version | 



## info_service_collect_event

> serde_json::Value info_service_collect_event(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodCollectEventRequest**](IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodCollectEventRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## info_service_get_info

> crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodInfoResponse info_service_get_info()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodInfoResponse**](io.argoproj.workflow.v1alpha1.InfoResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## info_service_get_user_info

> crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodGetUserInfoResponse info_service_get_user_info()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodGetUserInfoResponse**](io.argoproj.workflow.v1alpha1.GetUserInfoResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## info_service_get_version

> crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodVersion info_service_get_version()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodVersion**](io.argoproj.workflow.v1alpha1.Version.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

