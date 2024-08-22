# \PropertyControllerApi

All URIs are relative to *https://www.ebi.ac.uk/ols4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_properties**](PropertyControllerApi.md#get_all_properties) | **GET** /api/properties | 
[**get_properties_by_id_and_is_defining_ontology**](PropertyControllerApi.md#get_properties_by_id_and_is_defining_ontology) | **GET** /api/properties/findByIdAndIsDefiningOntology | 
[**get_properties_by_iri**](PropertyControllerApi.md#get_properties_by_iri) | **GET** /api/properties/{iri} | 
[**get_properties_by_iri_and_is_defining_ontology**](PropertyControllerApi.md#get_properties_by_iri_and_is_defining_ontology) | **GET** /api/properties/findByIdAndIsDefiningOntology/{iri} | 



## get_all_properties

> models::PagedModelV1Property get_all_properties(iri, short_form, obo_id, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iri** | Option<**String**> | The IRI of the property, this value must be double URL encoded |  |
**short_form** | Option<**String**> | This refers to the short form of the property. |  |
**obo_id** | Option<**String**> | This refers to the OBO ID of the property. |  |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Property**](PagedModelV1Property.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_properties_by_id_and_is_defining_ontology

> models::PagedModelV1Property get_properties_by_id_and_is_defining_ontology(iri, short_form, obo_id, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iri** | Option<**String**> | The IRI of the property, this value must be double URL encoded |  |
**short_form** | Option<**String**> | This refers to the short form of the property. |  |
**obo_id** | Option<**String**> | This refers to the OBO ID of the property. |  |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Property**](PagedModelV1Property.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_properties_by_iri

> models::PagedModelV1Property get_properties_by_iri(iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iri** | **String** | The IRI of the property, this value must be double URL encoded | [required] |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Property**](PagedModelV1Property.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_properties_by_iri_and_is_defining_ontology

> models::PagedModelV1Property get_properties_by_iri_and_is_defining_ontology(iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iri** | **String** | The IRI of the property, this value must be double URL encoded | [required] |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Property**](PagedModelV1Property.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

