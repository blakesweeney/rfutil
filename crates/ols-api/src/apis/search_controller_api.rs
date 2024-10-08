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

/// struct for typed errors of method [`search`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchError {
    Status404(),
    UnknownValue(serde_json::Value),
}

pub fn search(
    configuration: &configuration::Configuration,
    q: &str,
    ontology: Option<Vec<String>>,
    r#type: Option<Vec<String>>,
    slim: Option<Vec<String>>,
    field_list: Option<Vec<String>>,
    query_fields: Option<Vec<String>>,
    exact: Option<bool>,
    group_field: Option<&str>,
    obsoletes: Option<bool>,
    local: Option<bool>,
    children_of: Option<Vec<String>>,
    all_children_of: Option<Vec<String>>,
    inclusive: Option<bool>,
    is_leaf: Option<bool>,
    rows: Option<i32>,
    start: Option<i32>,
    format: Option<&str>,
    lang: Option<&str>,
) -> Result<(), Error<SearchError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/search", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("q", &q.to_string())]);
    if let Some(ref local_var_str) = ontology {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("ontology".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "ontology",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = r#type {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("type".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "type",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = slim {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("slim".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "slim",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = field_list {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("fieldList".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "fieldList",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = query_fields {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("queryFields".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "queryFields",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = exact {
        local_var_req_builder =
            local_var_req_builder.query(&[("exact", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = group_field {
        local_var_req_builder =
            local_var_req_builder.query(&[("groupField", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = obsoletes {
        local_var_req_builder =
            local_var_req_builder.query(&[("obsoletes", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = local {
        local_var_req_builder =
            local_var_req_builder.query(&[("local", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = children_of {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("childrenOf".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "childrenOf",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = all_children_of {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("allChildrenOf".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "allChildrenOf",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = inclusive {
        local_var_req_builder =
            local_var_req_builder.query(&[("inclusive", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_leaf {
        local_var_req_builder =
            local_var_req_builder.query(&[("isLeaf", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = rows {
        local_var_req_builder =
            local_var_req_builder.query(&[("rows", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start {
        local_var_req_builder =
            local_var_req_builder.query(&[("start", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = format {
        local_var_req_builder =
            local_var_req_builder.query(&[("format", &local_var_str.to_string())]);
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
        Ok(())
    } else {
        let local_var_entity: Option<SearchError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
