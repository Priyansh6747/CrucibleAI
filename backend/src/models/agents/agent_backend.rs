use crate::ai_functions::aifunc_backend::{print_backend_webserver_code, print_fixed_code, print_improved_webserver_code, print_rest_api_endpoints};
use crate::helper::general::{ai_task_req, read_exec_main_contents, save_backend_code};
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agent_basic::basic_traits::BasicTrait;
use crate::models::agents::agent_traits::{FactSheet, SpecialFunction};
use async_trait::async_trait;
use std::error::Error;

#[derive(Debug)]
pub struct AgentBackendDeveloper {
    attribute: BasicAgent,
    bug_error: Option<String>,
    bug_count: u8
}

impl AgentBackendDeveloper {
    pub fn new() -> Self {
        let attribute = BasicAgent{
            objective: "Develop backend code for webserver and json database".to_string(),
            position: "Backend Developer".to_string(),
            state : AgentState::Discovery,
            memory: vec![],
        };
        Self{
            attribute,
            bug_error:None,
            bug_count:0
        }
    }

    async fn call_initial_backend_code(&mut self ,fact_sheet: &mut FactSheet) {
        let msg_ctx = format!("Code template: {:?} Project Discription {:?}",
                                        fact_sheet.backend_code,fact_sheet);
        let ai_res = ai_task_req(
            msg_ctx,
            "Backend Developer",
            get_function_string!(print_backend_webserver_code),
            print_backend_webserver_code,
        ).await.get_string().unwrap();

        save_backend_code(&ai_res, "user1");
        fact_sheet.backend_code = Some(ai_res);
    }
    async fn call_improved_backend_code(&mut self ,fact_sheet: &mut FactSheet) {
        let msg_context: String = format!(
            "CODE TEMPLATE: {:?} \n PROJECT_DESCRIPTION: {:?} \n",
            fact_sheet.backend_code, fact_sheet
        );
        let ai_res = ai_task_req(
            msg_context,
            "Backend Developer",
            get_function_string!(print_improved_webserver_code),
            print_improved_webserver_code,
        ).await.get_string().unwrap();

        save_backend_code(&ai_res, "user1");
        fact_sheet.backend_code = Some(ai_res);
    }

    async fn call_fix_code_bugs(&mut self ,fact_sheet: &mut FactSheet) {
        let msg_ctx = format!("Broken Code: {:?} Error Bugs {:?} \n\
        Any output that have anything other than the relevant code will result in harsh punishments",
                              fact_sheet.backend_code,self.bug_error);
        let ai_res = ai_task_req(
            msg_ctx,
            "Backend Developer",
            get_function_string!(print_fixed_code),
            print_fixed_code,
        ).await.get_string().unwrap();

        save_backend_code(&ai_res, "user1");
        fact_sheet.backend_code = Some(ai_res);
    }
    async fn call_extract_rest_api_endpoints(&self) -> String {
        let backend_code: String = read_exec_main_contents("user1");

        // Structure message context
        let msg_context: String = format!("CODE_INPUT: {}", backend_code);

        let ai_res = ai_task_req(
            msg_context,
            &self.attribute.position,
            get_function_string!(print_rest_api_endpoints),
            print_rest_api_endpoints,
        ).await.get_string().unwrap();

        ai_res
    }

}

#[async_trait]
impl SpecialFunction for AgentBackendDeveloper {
    fn get_attribute_from_agent(&self) -> &BasicAgent {
        &self.attribute
    }
    async fn execute(&mut self, factsheet: &mut FactSheet) -> Result<(), Box<dyn Error + Send + Sync>> {
        while self.attribute.state != AgentState::Finished {
            match self.attribute.state {
                AgentState::Discovery => {
                    self.call_initial_backend_code(factsheet).await;
                    self.attribute.update_state(AgentState::Working);
                    continue;
                }
                AgentState::Working => {
                    if self.bug_count == 0 {}
                }
                AgentState::UnitTesting => {
                    todo!()
                }
                _ => {}
            }
        }
        Ok(())
    }
}