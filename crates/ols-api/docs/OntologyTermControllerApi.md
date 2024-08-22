# \OntologyTermControllerApi

All URIs are relative to *https://www.ebi.ac.uk/ols4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ancestors**](OntologyTermControllerApi.md#ancestors) | **GET** /api/ontologies/{onto}/terms/{iri}/ancestors | 
[**children**](OntologyTermControllerApi.md#children) | **GET** /api/ontologies/{onto}/terms/{iri}/children | 
[**descendants**](OntologyTermControllerApi.md#descendants) | **GET** /api/ontologies/{onto}/terms/{iri}/descendants | 
[**get_hierarchical_ancestors**](OntologyTermControllerApi.md#get_hierarchical_ancestors) | **GET** /api/ontologies/{onto}/terms/{iri}/hierarchicalAncestors | 
[**get_hierarchical_children**](OntologyTermControllerApi.md#get_hierarchical_children) | **GET** /api/ontologies/{onto}/terms/{iri}/hierarchicalChildren | 
[**get_hierarchical_descendants**](OntologyTermControllerApi.md#get_hierarchical_descendants) | **GET** /api/ontologies/{onto}/terms/{iri}/hierarchicalDescendants | 
[**get_hierarchical_parents**](OntologyTermControllerApi.md#get_hierarchical_parents) | **GET** /api/ontologies/{onto}/terms/{iri}/hierarchicalParents | 
[**get_parents**](OntologyTermControllerApi.md#get_parents) | **GET** /api/ontologies/{onto}/terms/{iri}/parents | 
[**get_preferred_roots**](OntologyTermControllerApi.md#get_preferred_roots) | **GET** /api/ontologies/{onto}/terms/preferredRoots | 
[**get_roots**](OntologyTermControllerApi.md#get_roots) | **GET** /api/ontologies/{onto}/terms/roots | 
[**get_term**](OntologyTermControllerApi.md#get_term) | **GET** /api/ontologies/{onto}/terms/{iri} | 
[**graph_js_tree**](OntologyTermControllerApi.md#graph_js_tree) | **GET** /api/ontologies/{onto}/terms/{iri}/jstree | 
[**graph_js_tree_children**](OntologyTermControllerApi.md#graph_js_tree_children) | **GET** /api/ontologies/{onto}/terms/{iri}/jstree/children/{nodeid} | 
[**graph_json**](OntologyTermControllerApi.md#graph_json) | **GET** /api/ontologies/{onto}/terms/{iri}/graph | 
[**related**](OntologyTermControllerApi.md#related) | **GET** /api/ontologies/{onto}/terms/{iri}/{property_iri} | 
[**term_ancestors_by_ontology**](OntologyTermControllerApi.md#term_ancestors_by_ontology) | **GET** /api/ontologies/{onto}/ancestors | 
[**term_children_by_ontology**](OntologyTermControllerApi.md#term_children_by_ontology) | **GET** /api/ontologies/{onto}/children | 
[**term_descendants_by_ontology**](OntologyTermControllerApi.md#term_descendants_by_ontology) | **GET** /api/ontologies/{onto}/descendants | 
[**term_hierarchical_ancestors_by_ontology**](OntologyTermControllerApi.md#term_hierarchical_ancestors_by_ontology) | **GET** /api/ontologies/{onto}/hierarchicalAncestors | 
[**term_hierarchical_children_by_ontology**](OntologyTermControllerApi.md#term_hierarchical_children_by_ontology) | **GET** /api/ontologies/{onto}/hierarchicalChildren | 
[**term_hierarchical_descendants_by_ontology**](OntologyTermControllerApi.md#term_hierarchical_descendants_by_ontology) | **GET** /api/ontologies/{onto}/hierarchicalDescendants | 
[**term_parents_by_ontology**](OntologyTermControllerApi.md#term_parents_by_ontology) | **GET** /api/ontologies/{onto}/parents | 
[**terms_by_ontology**](OntologyTermControllerApi.md#terms_by_ontology) | **GET** /api/ontologies/{onto}/terms | 



## ancestors

> models::PagedModelV1Term ancestors(onto, iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
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


## children

> models::PagedModelV1Term children(onto, iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
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


## descendants

> models::PagedModelV1Term descendants(onto, iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
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


## get_hierarchical_ancestors

> models::PagedModelV1Term get_hierarchical_ancestors(onto, iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
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


## get_hierarchical_children

> models::PagedModelV1Term get_hierarchical_children(onto, iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
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


## get_hierarchical_descendants

> models::PagedModelV1Term get_hierarchical_descendants(onto, iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
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


## get_hierarchical_parents

> models::PagedModelV1Term get_hierarchical_parents(onto, iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
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


## get_parents

> models::PagedModelV1Term get_parents(onto, iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
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


## get_preferred_roots

> models::PagedModelV1Term get_preferred_roots(onto, include_obsoletes, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
**include_obsoletes** | Option<**bool**> |  |  |[default to false]
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Term**](PagedModelV1Term.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_roots

> models::PagedModelV1Term get_roots(onto, include_obsoletes, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
**include_obsoletes** | Option<**bool**> |  |  |[default to false]
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Term**](PagedModelV1Term.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_term

> models::EntityModelV1Term get_term(onto, iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
**iri** | **String** | The IRI of the term, this value must be double URL encoded | [required] |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::EntityModelV1Term**](EntityModelV1Term.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## graph_js_tree

> String graph_js_tree(onto, iri, lang, siblings, view_mode)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
**iri** | **String** | The IRI of the term, this value must be double URL encoded | [required] |
**lang** | Option<**String**> |  |  |[default to en]
**siblings** | Option<**bool**> |  |  |[default to false]
**view_mode** | Option<**String**> |  |  |[default to PreferredRoots]

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## graph_js_tree_children

> String graph_js_tree_children(onto, iri, nodeid, lang)


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


## graph_json

> String graph_json(onto, iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
**iri** | **String** | The IRI of the term, this value must be double URL encoded | [required] |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## related

> models::PagedModelV1Term related(onto, iri, property_iri, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
**iri** | **String** | The IRI of the term, this value must be double URL encoded. | [required] |
**property_iri** | **String** | The IRI of the property, this must be double URL encoded. | [required] |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Term**](PagedModelV1Term.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## term_ancestors_by_ontology

> models::PagedModelV1Term term_ancestors_by_ontology(onto, iri, short_form, obo_id, id, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
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


## term_children_by_ontology

> models::PagedModelV1Term term_children_by_ontology(onto, iri, short_form, obo_id, id, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
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


## term_descendants_by_ontology

> models::PagedModelV1Term term_descendants_by_ontology(onto, iri, short_form, obo_id, id, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
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


## term_hierarchical_ancestors_by_ontology

> models::PagedModelV1Term term_hierarchical_ancestors_by_ontology(onto, iri, short_form, obo_id, id, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
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


## term_hierarchical_children_by_ontology

> models::PagedModelV1Term term_hierarchical_children_by_ontology(onto, iri, short_form, obo_id, id, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
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


## term_hierarchical_descendants_by_ontology

> models::PagedModelV1Term term_hierarchical_descendants_by_ontology(onto, iri, short_form, obo_id, id, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
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


## term_parents_by_ontology

> models::PagedModelV1Term term_parents_by_ontology(onto, iri, short_form, obo_id, id, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
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


## terms_by_ontology

> models::PagedModelV1Term terms_by_ontology(onto, iri, short_form, obo_id, id, obsoletes, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onto** | **String** | The ID of the ontology. For example for Data Use Ontology, the ID is duo. | [required] |
**iri** | Option<**String**> | The IRI of the term, this value must be double URL encoded |  |
**short_form** | Option<**String**> | This refers to the short form of the term. |  |
**obo_id** | Option<**String**> | This refers to the OBO ID of the term. |  |
**id** | Option<**String**> | This can be any of the above i.e. iri, short_form or obo_id. |  |
**obsoletes** | Option<**bool**> |  |  |
**lang** | Option<**String**> |  |  |[default to en]

### Return type

[**models::PagedModelV1Term**](PagedModelV1Term.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

