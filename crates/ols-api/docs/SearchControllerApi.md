# \SearchControllerApi

All URIs are relative to *https://www.ebi.ac.uk/ols4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search**](SearchControllerApi.md#search) | **GET** /api/search | 



## search

> search(q, ontology, r#type, slim, field_list, query_fields, exact, group_field, obsoletes, local, children_of, all_children_of, inclusive, is_leaf, rows, start, format, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** |  | [required] |
**ontology** | Option<[**Vec<String>**](String.md)> |  |  |
**r#type** | Option<[**Vec<String>**](String.md)> |  |  |
**slim** | Option<[**Vec<String>**](String.md)> |  |  |
**field_list** | Option<[**Vec<String>**](String.md)> |  |  |
**query_fields** | Option<[**Vec<String>**](String.md)> |  |  |
**exact** | Option<**bool**> |  |  |
**group_field** | Option<**String**> |  |  |
**obsoletes** | Option<**bool**> |  |  |[default to false]
**local** | Option<**bool**> |  |  |[default to false]
**children_of** | Option<[**Vec<String>**](String.md)> |  |  |
**all_children_of** | Option<[**Vec<String>**](String.md)> |  |  |
**inclusive** | Option<**bool**> |  |  |
**is_leaf** | Option<**bool**> |  |  |
**rows** | Option<**i32**> |  |  |[default to 10]
**start** | Option<**i32**> |  |  |[default to 0]
**format** | Option<**String**> |  |  |[default to json]
**lang** | Option<**String**> |  |  |[default to en]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

