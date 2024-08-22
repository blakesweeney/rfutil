# \OntologyIndividualControllerApi

All URIs are relative to *https://www.ebi.ac.uk/ols4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ancestors2**](OntologyIndividualControllerApi.md#ancestors2) | **GET** /api/ontologies/{onto}/individuals/{iri}/alltypes | 
[**get_all_individuals_by_ontology**](OntologyIndividualControllerApi.md#get_all_individuals_by_ontology) | **GET** /api/ontologies/{onto}/individuals | 
[**get_direct_types**](OntologyIndividualControllerApi.md#get_direct_types) | **GET** /api/ontologies/{onto}/individuals/{iri}/types | 
[**get_individual**](OntologyIndividualControllerApi.md#get_individual) | **GET** /api/ontologies/{onto}/individuals/{iri} | 
[**get_js_tree1**](OntologyIndividualControllerApi.md#get_js_tree1) | **GET** /api/ontologies/{onto}/individuals/{iri}/jstree | 



## ancestors2

> models::PagedModelV1Property ancestors2(onto, iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Information Artifact Ontology, the ID is iao. | [required] |
**iri** | **String** | The IRI of the individual, this IRI should exist in the specified ontology by {onto} param. This value must be double URL encoded | [required] |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Property**](PagedModelV1Property.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_individuals_by_ontology

> models::PagedModelV1Individual get_all_individuals_by_ontology(onto, iri, short_form, obo_id, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Information Artifact Ontology, the ID is iao. | [required] |
**iri** | Option<**String**> | The IRI of the individual, this IRI should exist in the specified ontology by {onto} param. This value must be double URL encoded |  |
**short_form** | Option<**String**> | This refers to the short form of the individual, it should exist in the specified ontology by {onto} param. |  |
**obo_id** | Option<**String**> | This refers to the OBO ID of the individual, it should exist in the specified ontology by {onto} param. |  |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Individual**](PagedModelV1Individual.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_direct_types

> models::PagedModelV1Term get_direct_types(onto, iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Information Artifact Ontology, the ID is iao. | [required] |
**iri** | **String** | The IRI of the individual, this IRI should exist in the specified ontology by {onto} param. This value must be double URL encoded | [required] |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Term**](PagedModelV1Term.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_individual

> models::EntityModelV1Individual get_individual(onto, iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Information Artifact Ontology, the ID is iao. | [required] |
**iri** | **String** | The IRI of the individual, this IRI should exist in the specified ontology by {onto} param. This value must be double URL encoded | [required] |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::EntityModelV1Individual**](EntityModelV1Individual.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_js_tree1

> String get_js_tree1(onto, iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Information Artifact Ontology, the ID is iao. | [required] |
**iri** | **String** | The IRI of the individual, this IRI should exist in the specified ontology by {onto} param. This value must be double URL encoded | [required] |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

