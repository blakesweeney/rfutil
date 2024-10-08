/*
 * OLS Service
 *
 * REST API for OLS
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityModelV1Individual {
    #[serde(rename = "iri", skip_serializing_if = "Option::is_none")]
    pub iri: Option<String>,
    #[serde(rename = "lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    #[serde(rename = "synonyms", skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Vec<String>>,
    #[serde(rename = "annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Vec<models::V1Term>>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "ontology_name", skip_serializing_if = "Option::is_none")]
    pub ontology_name: Option<String>,
    #[serde(rename = "ontology_prefix", skip_serializing_if = "Option::is_none")]
    pub ontology_prefix: Option<String>,
    #[serde(rename = "ontology_iri", skip_serializing_if = "Option::is_none")]
    pub ontology_iri: Option<String>,
    #[serde(rename = "is_obsolete", skip_serializing_if = "Option::is_none")]
    pub is_obsolete: Option<bool>,
    #[serde(
        rename = "is_defining_ontology",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_defining_ontology: Option<bool>,
    #[serde(rename = "has_children", skip_serializing_if = "Option::is_none")]
    pub has_children: Option<bool>,
    #[serde(rename = "is_root", skip_serializing_if = "Option::is_none")]
    pub is_root: Option<bool>,
    #[serde(rename = "short_form", skip_serializing_if = "Option::is_none")]
    pub short_form: Option<String>,
    #[serde(rename = "obo_id", skip_serializing_if = "Option::is_none")]
    pub obo_id: Option<String>,
    #[serde(rename = "in_subset", skip_serializing_if = "Option::is_none")]
    pub in_subset: Option<Vec<String>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<models::Link>>,
}

impl EntityModelV1Individual {
    pub fn new() -> EntityModelV1Individual {
        EntityModelV1Individual {
            iri: None,
            lang: None,
            description: None,
            synonyms: None,
            annotation: None,
            r#type: None,
            label: None,
            ontology_name: None,
            ontology_prefix: None,
            ontology_iri: None,
            is_obsolete: None,
            is_defining_ontology: None,
            has_children: None,
            is_root: None,
            short_form: None,
            obo_id: None,
            in_subset: None,
            links: None,
        }
    }
}
