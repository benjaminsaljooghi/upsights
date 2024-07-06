# \DatabasesApi

All URIs are relative to *https://api.notion.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_databases_id_get**](DatabasesApi.md#v1_databases_id_get) | **GET** /v1/databases/{id} | Retrieve a database
[**v1_databases_id_patch**](DatabasesApi.md#v1_databases_id_patch) | **PATCH** /v1/databases/{id} | Update database properties
[**v1_databases_id_query_post**](DatabasesApi.md#v1_databases_id_query_post) | **POST** /v1/databases/{id}/query | Filter a database
[**v1_databases_post**](DatabasesApi.md#v1_databases_post) | **POST** /v1/databases/ | Create a database



## v1_databases_id_get

> serde_json::Value v1_databases_id_get(id, notion_version)
Retrieve a database

Retrieves a database object using the ID specified in the request path. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**notion_version** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_databases_id_patch

> serde_json::Value v1_databases_id_patch(id, notion_version, body)
Update database properties

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**notion_version** | Option<**String**> |  |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_databases_id_query_post

> serde_json::Value v1_databases_id_query_post(id, authorization, content_type, notion_version, body)
Filter a database

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**authorization** | Option<**String**> |  |  |
**content_type** | Option<**String**> |  |  |
**notion_version** | Option<**String**> |  |  |
**body** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_databases_post

> serde_json::Value v1_databases_post(content_type, notion_version, body)
Create a database

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_type** | Option<**String**> |  |  |
**notion_version** | Option<**String**> |  |  |
**body** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

