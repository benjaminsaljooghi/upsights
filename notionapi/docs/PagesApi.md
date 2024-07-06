# \PagesApi

All URIs are relative to *https://api.notion.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_pages_id_get**](PagesApi.md#v1_pages_id_get) | **GET** /v1/pages/{id} | Retrieve a page
[**v1_pages_id_patch**](PagesApi.md#v1_pages_id_patch) | **PATCH** /v1/pages/{id} | Archive a page
[**v1_pages_page_id_properties_property_id_get**](PagesApi.md#v1_pages_page_id_properties_property_id_get) | **GET** /v1/pages/{page_id}/properties/{property_id} | Retrieve a page property item
[**v1_pages_post**](PagesApi.md#v1_pages_post) | **POST** /v1/pages/ | Create a page with content



## v1_pages_id_get

> serde_json::Value v1_pages_id_get(id, notion_version)
Retrieve a page

Retrieves a Page object using the ID in the request path. This endpoint exposes page properties, not page content. 

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


## v1_pages_id_patch

> serde_json::Value v1_pages_id_patch(id, notion_version, body)
Archive a page

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


## v1_pages_page_id_properties_property_id_get

> serde_json::Value v1_pages_page_id_properties_property_id_get(page_id, property_id, notion_version)
Retrieve a page property item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_id** | **String** |  | [required] |
**property_id** | **String** |  | [required] |
**notion_version** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_pages_post

> serde_json::Value v1_pages_post(authorization, content_type, notion_version, body)
Create a page with content

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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

