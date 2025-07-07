use crate::ai_functions::aifunc_architect::{print_project_scope, print_site_urls};
use crate::helper::command_line::PrinCommand;
use crate::helper::general::{ai_task_req_decoded, check_status_code};
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agent_basic::basic_traits::BasicTrait;
use crate::models::agents::agent_traits::{FactSheet, ProjectScope, SpecialFunction};
use async_trait::async_trait;
use std::error::Error;
use std::time::Duration;

#[derive(Debug)]
pub struct AgentSolutionrAchitect{
    attribute: BasicAgent,
}

impl AgentSolutionrAchitect {
    pub fn new() -> Self {
        let attribute = BasicAgent{
            objective: "Gathers information and design solution for website development".to_string(),
            position: "Solution Architect".to_string(),
            state : AgentState::Discovery,
            memory: vec![],
        };
        Self{attribute}
    }

    async fn call_project_scope(&mut self , fact_sheet: &mut FactSheet)->ProjectScope {
        let msg_ctx = format!("{}",fact_sheet.project_disc);
        let ai_response: ProjectScope = ai_task_req_decoded::<ProjectScope>(
            msg_ctx,
            &self.attribute.position,
            get_function_string!(print_project_scope),
            print_project_scope,
        ).await;
        fact_sheet.project_scope = Some(ai_response.clone());
        self.attribute.update_state(AgentState::Finished);
        ai_response
    }

    async fn call_determine_external_url(&mut self , fact_sheet: &mut FactSheet , msg_ctx: String){
        let ai_response: Vec<String> = ai_task_req_decoded::<Vec<String>>(
            msg_ctx,
            &self.attribute.position,
            get_function_string!(print_site_urls),
            print_site_urls,
        ).await;

        fact_sheet.external_urls = Some(ai_response);
        self.attribute.update_state(AgentState::UnitTesting);
    }
}

#[async_trait]
impl SpecialFunction for AgentSolutionrAchitect {
    fn get_attribute_from_agent(&self) -> &BasicAgent {
        &self.attribute
    }

    async fn execute(&mut self, factsheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        while self.attribute.state != AgentState::Finished {
            match self.attribute.state {
                AgentState::Discovery => {
                    let project_scope = self.call_project_scope(factsheet).await;
                    if project_scope.is_external_urls_required {
                        self.call_determine_external_url(factsheet,factsheet.project_disc.clone()).await;
                        self.attribute.state = AgentState::UnitTesting;
                    }
                }
                AgentState::UnitTesting => {
                    let mut exclude_url:Vec<String> = Vec::new();
                    let client = reqwest::Client::builder()
                        .timeout(Duration::from_secs(5))
                        .build()
                        .unwrap();

                    let urls = factsheet.external_urls.as_ref()
                        .expect("No URL object on factsheet");

                    for url in urls {
                        let endpoint = format!("Testing URL {}", url);
                        PrinCommand::UnitTest.print_agent_message(self.attribute.position.as_str(), endpoint.as_str());
                        match check_status_code(&client,url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    exclude_url.push(url.clone());
                                }
                            }
                            Err(e) => {
                                println!("Error Checking: {} {}", url, e);
                            }
                        }
                    };
                    if exclude_url.len() > 0 {
                        let new_urls:Vec<String> = factsheet.external_urls.as_ref().unwrap().iter()
                            .filter(|url| !exclude_url.contains(url))
                            .cloned().collect();
                        factsheet.external_urls = Some(new_urls);
                    }

                    self.attribute.update_state(AgentState::Finished);
                }
                _ => {
                    self.attribute.state = AgentState::Finished;
                }
            }
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_sol_architect(){
        let mut architect = AgentSolutionrAchitect::new();
        let mut fact = FactSheet{
            project_disc: "Build a stock website with user login and logout that shows lates forex prices".to_string(),
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema:None
        };
        architect.execute(&mut fact).await.expect("Unable to execute");
        assert!(fact.project_scope.is_some());
        assert!(fact.external_urls.is_some());
        dbg!(fact);
    }
}