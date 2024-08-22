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
pub struct V1OboXref {
    #[serde(rename = "database", skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl V1OboXref {
    pub fn new() -> V1OboXref {
        V1OboXref {
            database: None,
            id: None,
            description: None,
            url: None,
        }
    }
}

