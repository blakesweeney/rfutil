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
pub struct EntityModelV1Ontology {
    #[serde(rename = "languages", skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    #[serde(rename = "lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(rename = "ontologyId", skip_serializing_if = "Option::is_none")]
    pub ontology_id: Option<String>,
    #[serde(rename = "loaded", skip_serializing_if = "Option::is_none")]
    pub loaded: Option<String>,
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "fileHash", skip_serializing_if = "Option::is_none")]
    pub file_hash: Option<String>,
    #[serde(rename = "loadAttempts", skip_serializing_if = "Option::is_none")]
    pub load_attempts: Option<i32>,
    #[serde(rename = "numberOfTerms", skip_serializing_if = "Option::is_none")]
    pub number_of_terms: Option<i32>,
    #[serde(rename = "numberOfProperties", skip_serializing_if = "Option::is_none")]
    pub number_of_properties: Option<i32>,
    #[serde(
        rename = "numberOfIndividuals",
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_individuals: Option<i32>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<models::V1OntologyConfig>>,
    #[serde(rename = "baseUris", skip_serializing_if = "Option::is_none")]
    pub base_uris: Option<Vec<String>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<models::Link>>,
}

impl EntityModelV1Ontology {
    pub fn new() -> EntityModelV1Ontology {
        EntityModelV1Ontology {
            languages: None,
            lang: None,
            ontology_id: None,
            loaded: None,
            updated: None,
            status: None,
            message: None,
            version: None,
            file_hash: None,
            load_attempts: None,
            number_of_terms: None,
            number_of_properties: None,
            number_of_individuals: None,
            config: None,
            base_uris: None,
            links: None,
        }
    }
}
