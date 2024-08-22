# \IndividualControllerApi

All URIs are relative to *https://www.ebi.ac.uk/ols4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_individuals**](IndividualControllerApi.md#get_all_individuals) | **GET** /api/individuals | 
[**get_all_individuals1**](IndividualControllerApi.md#get_all_individuals1) | **GET** /api/individuals/{iri} | 
[**get_all_individuals_by_id_and_is_defining_ontology**](IndividualControllerApi.md#get_all_individuals_by_id_and_is_defining_ontology) | **GET** /api/individuals/findByIdAndIsDefiningOntology | 
[**get_all_individuals_by_id_and_is_defining_ontology1**](IndividualControllerApi.md#get_all_individuals_by_id_and_is_defining_ontology1) | **GET** /api/individuals/findByIdAndIsDefiningOntology/{iri} | 



## get_all_individuals

> models::PagedModelV1Individual get_all_individuals(iri, short_form, obo_id, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iri** | Option<**String**> | The IRI of the individual, this value must be double URL encoded |  |
**short_form** | Option<**String**> | This refers to the short form of the individual. |  |
**obo_id** | Option<**String**> | This refers to the OBO ID of the individual. |  |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Individual**](PagedModelV1Individual.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_individuals1

> models::PagedModelV1Individual get_all_individuals1(iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iri** | **String** | The IRI of the individual, this value must be double URL encoded | [required] |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Individual**](PagedModelV1Individual.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_individuals_by_id_and_is_defining_ontology

> models::PagedModelV1Individual get_all_individuals_by_id_and_is_defining_ontology(iri, short_form, obo_id, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iri** | Option<**String**> | The IRI of the individual, this value must be double URL encoded |  |
**short_form** | Option<**String**> | This refers to the short form of the individual. |  |
**obo_id** | Option<**String**> | This refers to the OBO ID of the individual. |  |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Individual**](PagedModelV1Individual.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_individuals_by_id_and_is_defining_ontology1

> models::PagedModelV1Individual get_all_individuals_by_id_and_is_defining_ontology1(iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iri** | **String** | The IRI of the individual, this value must be double URL encoded | [required] |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Individual**](PagedModelV1Individual.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

