# \OntologyPropertyControllerApi

All URIs are relative to *https://www.ebi.ac.uk/ols4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ancestors1**](OntologyPropertyControllerApi.md#ancestors1) | **GET** /api/ontologies/{onto}/properties/{iri}/ancestors | 
[**children1**](OntologyPropertyControllerApi.md#children1) | **GET** /api/ontologies/{onto}/properties/{iri}/children | 
[**descendants1**](OntologyPropertyControllerApi.md#descendants1) | **GET** /api/ontologies/{onto}/properties/{iri}/descendants | 
[**get_all_properties_by_ontology**](OntologyPropertyControllerApi.md#get_all_properties_by_ontology) | **GET** /api/ontologies/{onto}/properties | 
[**get_js_tree**](OntologyPropertyControllerApi.md#get_js_tree) | **GET** /api/ontologies/{onto}/properties/{iri}/jstree | 
[**get_parents1**](OntologyPropertyControllerApi.md#get_parents1) | **GET** /api/ontologies/{onto}/properties/{iri}/parents | 
[**get_property**](OntologyPropertyControllerApi.md#get_property) | **GET** /api/ontologies/{onto}/properties/{iri} | 
[**get_roots1**](OntologyPropertyControllerApi.md#get_roots1) | **GET** /api/ontologies/{onto}/properties/roots | 
[**graph_js_tree_children1**](OntologyPropertyControllerApi.md#graph_js_tree_children1) | **GET** /api/ontologies/{onto}/properties/{iri}/jstree/children/{nodeid} | 



## ancestors1

> models::PagedModelV1Property ancestors1(onto, iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
**iri** | **String** | The IRI of the property, this IRI should exist in the specified ontology by {onto} param. This value must be double URL encoded | [required] |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Property**](PagedModelV1Property.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## children1

> models::PagedModelV1Property children1(onto, iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
**iri** | **String** | The IRI of the property, this IRI should exist in the specified ontology by {onto} param. This value must be double URL encoded | [required] |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Property**](PagedModelV1Property.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## descendants1

> models::PagedModelV1Property descendants1(onto, iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
**iri** | **String** | The IRI of the property, this IRI should exist in the specified ontology by {onto} param. This value must be double URL encoded | [required] |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Property**](PagedModelV1Property.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_properties_by_ontology

> models::PagedModelV1Property get_all_properties_by_ontology(onto, iri, short_form, obo_id, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
**iri** | Option<**String**> | The IRI of the property, this IRI should exist in the specified ontology by {onto} param. This value must be double URL encoded |  |
**short_form** | Option<**String**> | This refers to the short form of the property, it should exist in the specified ontology by {onto} param. |  |
**obo_id** | Option<**String**> | This refers to the OBO ID of the property, it should exist in the specified ontology by {onto} param. |  |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Property**](PagedModelV1Property.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_js_tree

> String get_js_tree(onto, iri, siblings, view_mode, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
**iri** | **String** | The IRI of the property, this IRI should exist in the specified ontology by {onto} param. This value must be double URL encoded | [required] |
**siblings** | Option<**bool**> |  |  |[default to false]
**view_mode** | Option<**String**> |  |  |[default to PreferredRoots]
**lang** | Option<**String**> |  |  |[default to en]

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_parents1

> models::PagedModelV1Property get_parents1(onto, iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
**iri** | **String** | The IRI of the property, this IRI should exist in the specified ontology by {onto} param. This value must be double URL encoded | [required] |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Property**](PagedModelV1Property.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_property

> models::EntityModelV1Property get_property(onto, iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
**iri** | **String** | The IRI of the property, this IRI should exist in the specified ontology by {onto} param. This value must be double URL encoded | [required] |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::EntityModelV1Property**](EntityModelV1Property.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_roots1

> models::PagedModelV1Property get_roots1(onto, include_obsoletes, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
**include_obsoletes** | Option<**bool**> | A boolean flag to get Obsolete terms |  |[default to false]
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Property**](PagedModelV1Property.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## graph_js_tree_children1

> String graph_js_tree_children1(onto, iri, nodeid, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
**iri** | **String** | The IRI of the property, this IRI should exist in the specified ontology by {onto} param. This value must be double URL encoded | [required] |
**nodeid** | **String** | This is the id of the node in the jstree of ontology specified by {onto} parameter | [required] |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

