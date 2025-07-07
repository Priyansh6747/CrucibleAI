use crate::models::agent_basic::basic_agent;
use async_trait::async_trait;
use serde::{ Serialize, Deserialize };
use std::fmt::Debug;
use crate::models::agent_basic::basic_agent::BasicAgent;

#[derive(Deserialize, Serialize, Debug, Clone,PartialEq)]
pub struct RouteObject {
    pub is_route_dynamic: String,
    pub method: String,
    pub request_body: serde_json::Value,
    pub response: serde_json::Value,
    pub route: String,
}

#[derive(Deserialize, Serialize, Debug, Clone,PartialEq,Copy)]
pub struct ProjectScope{
    pub is_crud_required : bool,
    pub is_user_login_and_logout: bool,
    pub is_external_urls_required : bool,
}

#[derive(Deserialize, Serialize, Debug, Clone,PartialEq)]
pub struct FactSheet{
    pub project_disc:String,
    pub project_scope:Option<ProjectScope>,
    pub external_urls:Option<Vec<String>>,
    pub backend_code:Option<String>,
    pub api_endpoint_schema:Option<Vec<RouteObject>>,
}
#[async_trait]
pub trait SpecialFunction: Debug {
    fn get_attribute_from_agent(&self) -> &BasicAgent;
    async fn execute(&mut self, factsheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error + Send + Sync>> ;
}

impl FactSheet {
    pub(crate) fn default() -> FactSheet {
        FactSheet{
            project_disc: "Test".to_string(),
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,
        }
    }
}