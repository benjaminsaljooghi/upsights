# \BlocksApi

All URIs are relative to *https://api.notion.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_blocks_id_children_get**](BlocksApi.md#v1_blocks_id_children_get) | **GET** /v1/blocks/{id}/children | Retrieve block children
[**v1_blocks_id_children_patch**](BlocksApi.md#v1_blocks_id_children_patch) | **PATCH** /v1/blocks/{id}/children | Append block children
[**v1_blocks_id_delete**](BlocksApi.md#v1_blocks_id_delete) | **DELETE** /v1/blocks/{id} | Delete a block
[**v1_blocks_id_get**](BlocksApi.md#v1_blocks_id_get) | **GET** /v1/blocks/{id} | Retrieve a block
[**v1_blocks_id_patch**](BlocksApi.md#v1_blocks_id_patch) | **PATCH** /v1/blocks/{id} | Update a block



## v1_blocks_id_children_get

> serde_json::Value v1_blocks_id_children_get(id, notion_version, page_size)
Retrieve block children

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**notion_version** | Option<**String**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_blocks_id_children_patch

> serde_json::Value v1_blocks_id_children_patch(id, authorization, content_type, notion_version, body)
Append block children

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


## v1_blocks_id_delete

> serde_json::Value v1_blocks_id_delete(id, notion_version)
Delete a block

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


## v1_blocks_id_get

> serde_json::Value v1_blocks_id_get(id, notion_version)
Retrieve a block

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


## v1_blocks_id_patch

> serde_json::Value v1_blocks_id_patch(id, notion_version, body)
Update a block

This endpoint allows you to update block content. [See Full Documentation](https://developers.notion.com/reference/update-a-block)

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

