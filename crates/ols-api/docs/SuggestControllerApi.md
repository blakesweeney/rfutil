# \SuggestControllerApi

All URIs are relative to *https://www.ebi.ac.uk/ols4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**suggest**](SuggestControllerApi.md#suggest) | **GET** /api/suggest | 



## suggest

> suggest(q, ontology, rows, start)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** |  | [required] |
**ontology** | Option<[**Vec<String>**](String.md)> |  |  |
**rows** | Option<**i32**> |  |  |[default to 10]
**start** | Option<**i32**> |  |  |[default to 0]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

