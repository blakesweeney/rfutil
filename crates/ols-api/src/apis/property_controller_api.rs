/*
 * OLS Service
 *
 * REST API for OLS
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};

/// struct for typed errors of method [`get_all_properties`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllPropertiesError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_properties_by_id_and_is_defining_ontology`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPropertiesByIdAndIsDefiningOntologyError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_properties_by_iri`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPropertiesByIriError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_properties_by_iri_and_is_defining_ontology`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPropertiesByIriAndIsDefiningOntologyError {
    Status404(),
    UnknownValue(serde_json::Value),
}

pub fn get_all_properties(
    configuration: &configuration::Configuration,
    iri: Option<&str>,
    short_form: Option<&str>,
    obo_id: Option<&str>,
    lang: Option<&str>,
) -> Result<models::PagedModelV1Property, Error<GetAllPropertiesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/properties", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = iri {
        local_var_req_builder = local_var_req_builder.query(&[("iri", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = short_form {
        local_var_req_builder =
            local_var_req_builder.query(&[("short_form", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = obo_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("obo_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = lang {
        local_var_req_builder =
            local_var_req_builder.query(&[("lang", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetAllPropertiesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn get_properties_by_id_and_is_defining_ontology(
    configuration: &configuration::Configuration,
    iri: Option<&str>,
    short_form: Option<&str>,
    obo_id: Option<&str>,
    lang: Option<&str>,
) -> Result<models::PagedModelV1Property, Error<GetPropertiesByIdAndIsDefiningOntologyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/properties/findByIdAndIsDefiningOntology",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = iri {
        local_var_req_builder = local_var_req_builder.query(&[("iri", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = short_form {
        local_var_req_builder =
            local_var_req_builder.query(&[("short_form", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = obo_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("obo_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = lang {
        local_var_req_builder =
            local_var_req_builder.query(&[("lang", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetPropertiesByIdAndIsDefiningOntologyError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn get_properties_by_iri(
    configuration: &configuration::Configuration,
    iri: &str,
    lang: Option<&str>,
) -> Result<models::PagedModelV1Property, Error<GetPropertiesByIriError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/properties/{iri}",
        local_var_configuration.base_path,
        iri = crate::apis::urlencode(iri)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = lang {
        local_var_req_builder =
            local_var_req_builder.query(&[("lang", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetPropertiesByIriError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn get_properties_by_iri_and_is_defining_ontology(
    configuration: &configuration::Configuration,
    iri: &str,
    lang: Option<&str>,
) -> Result<models::PagedModelV1Property, Error<GetPropertiesByIriAndIsDefiningOntologyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/properties/findByIdAndIsDefiningOntology/{iri}",
        local_var_configuration.base_path,
        iri = crate::apis::urlencode(iri)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = lang {
        local_var_req_builder =
            local_var_req_builder.query(&[("lang", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetPropertiesByIriAndIsDefiningOntologyError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
