# \UsersApi

All URIs are relative to *https://api.notion.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_users_get**](UsersApi.md#v1_users_get) | **GET** /v1/users | List all users
[**v1_users_id_get**](UsersApi.md#v1_users_id_get) | **GET** /v1/users/{id} | Retrieve a user
[**v1_users_me_get**](UsersApi.md#v1_users_me_get) | **GET** /v1/users/me | Retrieve your token’s bot user



## v1_users_get

> serde_json::Value v1_users_get(notion_version)
List all users

Returns a paginated list of user objects for a workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notion_version** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_users_id_get

> serde_json::Value v1_users_id_get(id, notion_version)
Retrieve a user

Retrieve a user object using the ID specified in the request path.

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


## v1_users_me_get

> serde_json::Value v1_users_me_get(notion_version)
Retrieve your token’s bot user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notion_version** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

