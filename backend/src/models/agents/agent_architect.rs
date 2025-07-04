use crate::ai_functions::aifunc_architect::print_project_scope;
use crate::helper::general::{ai_task_req, ai_task_req_decoded};
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agent_basic::basic_traits::BasicTrait;
use crate::models::agents::agent_traits::{FactSheet, ProjectScope};

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
}