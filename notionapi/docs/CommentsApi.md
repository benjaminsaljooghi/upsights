# \CommentsApi

All URIs are relative to *https://api.notion.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_comments_get**](CommentsApi.md#v1_comments_get) | **GET** /v1/comments | Retrieve comments
[**v1_comments_post**](CommentsApi.md#v1_comments_post) | **POST** /v1/comments | Add comment to discussion



## v1_comments_get

> serde_json::Value v1_comments_get(notion_version, block_id, page_size)
Retrieve comments

Retrieve a user object using the ID specified in the request path.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notion_version** | Option<**String**> |  |  |
**block_id** | Option<**String**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_comments_post

> serde_json::Value v1_comments_post(content_type, notion_version, body)
Add comment to discussion

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

