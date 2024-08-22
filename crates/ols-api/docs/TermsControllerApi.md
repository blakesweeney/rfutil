# \TermsControllerApi

All URIs are relative to *https://www.ebi.ac.uk/ols4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_terms**](TermsControllerApi.md#get_terms) | **GET** /api/terms | 
[**get_terms_by_id_and_is_defining_ontology**](TermsControllerApi.md#get_terms_by_id_and_is_defining_ontology) | **GET** /api/terms/findByIdAndIsDefiningOntology | 
[**get_terms_by_id_and_is_defining_ontology1**](TermsControllerApi.md#get_terms_by_id_and_is_defining_ontology1) | **GET** /api/terms/findByIdAndIsDefiningOntology/{iri} | 
[**get_terms_by_iri**](TermsControllerApi.md#get_terms_by_iri) | **GET** /api/terms/{iri} | 



## get_terms

> models::PagedModelV1Term get_terms(iri, short_form, obo_id, id, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iri** | Option<**String**> | The IRI of the term, this value must be double URL encoded |  |
**short_form** | Option<**String**> | This refers to the short form of the term. |  |
**obo_id** | Option<**String**> | This refers to the OBO ID of the term. |  |
**id** | Option<**String**> | This can be any of the above i.e. iri, short_form or obo_id. |  |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Term**](PagedModelV1Term.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_terms_by_id_and_is_defining_ontology

> models::PagedModelV1Term get_terms_by_id_and_is_defining_ontology(iri, short_form, obo_id, id, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iri** | Option<**String**> | The IRI of the term, this value must be double URL encoded |  |
**short_form** | Option<**String**> | This refers to the short form of the term. |  |
**obo_id** | Option<**String**> | This refers to the OBO ID of the term. |  |
**id** | Option<**String**> | This can be any of the above i.e. iri, short_form or obo_id. |  |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Term**](PagedModelV1Term.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_terms_by_id_and_is_defining_ontology1

> models::PagedModelV1Term get_terms_by_id_and_is_defining_ontology1(iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iri** | **String** | The IRI of the term, this value must be double URL encoded | [required] |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Term**](PagedModelV1Term.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_terms_by_iri

> models::PagedModelV1Term get_terms_by_iri(iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iri** | **String** | The IRI of the term, this value must be double URL encoded | [required] |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Term**](PagedModelV1Term.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

