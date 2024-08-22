# \OntologyControllerApi

All URIs are relative to *https://www.ebi.ac.uk/ols4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_ontologies**](OntologyControllerApi.md#get_ontologies) | **GET** /api/ontologies | 
[**get_ontology**](OntologyControllerApi.md#get_ontology) | **GET** /api/ontologies/{onto} | 



## get_ontologies

> models::PagedModelV1Ontology get_ontologies(lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Ontology**](PagedModelV1Ontology.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ontology

> models::EntityModelV1Ontology get_ontology(onto, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::EntityModelV1Ontology**](EntityModelV1Ontology.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

