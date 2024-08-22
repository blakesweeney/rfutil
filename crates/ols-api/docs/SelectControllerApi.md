# \SelectControllerApi

All URIs are relative to *https://www.ebi.ac.uk/ols4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**select**](SelectControllerApi.md#select) | **GET** /api/select | 



## select

> select(q, ontology, r#type, slim, field_list, obsoletes, local, children_of, all_children_of, rows, start, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** |  | [required] |
**ontology** | Option<[**Vec<String>**](String.md)> |  |  |
**r#type** | Option<[**Vec<String>**](String.md)> |  |  |
**slim** | Option<[**Vec<String>**](String.md)> |  |  |
**field_list** | Option<[**Vec<String>**](String.md)> |  |  |
**obsoletes** | Option<**bool**> |  |  |[default to false]
**local** | Option<**bool**> |  |  |[default to false]
**children_of** | Option<[**Vec<String>**](String.md)> |  |  |
**all_children_of** | Option<[**Vec<String>**](String.md)> |  |  |
**rows** | Option<**i32**> |  |  |[default to 10]
**start** | Option<**i32**> |  |  |[default to 0]
**lang** | Option<**String**> |  |  |[default to en]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

