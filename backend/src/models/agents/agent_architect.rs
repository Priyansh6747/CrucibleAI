use std::error::Error;
use async_trait::async_trait;
use crate::ai_functions::aifunc_architect::{print_project_scope, print_site_urls};
use crate::helper::general::{ai_task_req, ai_task_req_decoded};
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agent_basic::basic_traits::BasicTrait;
use crate::models::agents::agent_traits::{FactSheet, ProjectScope, SpecialFunction};

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
                    let _p = self.call_project_scope(factsheet).await;
                }
                AgentState::UnitTesting => {
                    todo!()
                }
                _ => {
                    self.attribute.state = AgentState::Finished;
                }
            }
        }

        Ok(())
    }
}

// Add this to your code to systematically check what's not Send
use std::marker::Send;

// Test each type individually
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

#[test]
fn debug_send_types() {
    // Test your basic types
    assert_send::<FactSheet>();
    assert_send::<AgentSolutionrAchitect>();
    assert_send::<BasicAgent>();
    assert_send::<AgentState>();
    assert_send::<ProjectScope>();

    // Test the function types
    assert_send::<fn(&str) -> &'static str>();
    

    
}

#[tokio::test]
async fn test_send_across_threads() {
    let mut agent = AgentSolutionrAchitect::new();
    let mut factsheet = FactSheet::default(); // Assuming it has Default

    // This should fail if not Send
    tokio::spawn(async move {
        let _ = agent.execute(&mut factsheet).await;
    });
}