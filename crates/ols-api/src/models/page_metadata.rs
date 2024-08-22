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
pub struct PageMetadata {
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "totalElements", skip_serializing_if = "Option::is_none")]
    pub total_elements: Option<i64>,
    #[serde(rename = "totalPages", skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i64>,
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
}

impl PageMetadata {
    pub fn new() -> PageMetadata {
        PageMetadata {
            size: None,
            total_elements: None,
            total_pages: None,
            number: None,
        }
    }
}
