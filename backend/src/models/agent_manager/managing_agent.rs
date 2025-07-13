use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;
use crate::helper::general::{ai_task_req, save_api_json};
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agents::agent_architect::AgentSolutionrAchitect;
use crate::models::agents::agent_backend::AgentBackendDeveloper;
use crate::models::agents::agent_traits::{FactSheet, SpecialFunction};

pub struct ManagingAgent {
    attribute: BasicAgent,
    factsheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunction>>,
}

impl ManagingAgent {
    pub async fn new(user_req:String)->Result<Self,Box<dyn std::error::Error>>{
        let attribute = BasicAgent{
            objective: "Manage agents who are building an excellent website for the user".to_string(),
            position: "Project Manager".to_string(),
            state : AgentState::Discovery,
            memory: vec![],
        };
        let project_disc = ai_task_req(
            user_req,
            "Project Manager",
            get_function_string!(convert_user_input_to_goal),
            convert_user_input_to_goal,
        ).await.get_string().unwrap();

        let agents: Vec<Box<dyn SpecialFunction>> = vec![];
        let factsheet = FactSheet{
            project_disc,
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema:None
        };

        Ok(Self{
            attribute,
            factsheet,
            agents,
        })
    }

    fn add_agent(&mut self, agent: Box<dyn SpecialFunction>){
        self.agents.push(agent);
    }
    fn create_agents(&mut self){
        self.add_agent(Box::new(AgentSolutionrAchitect::new()));
        self.add_agent(Box::new(AgentBackendDeveloper::new()))
    }
    
    pub async fn execute_project(&mut self){
        self.create_agents();
        for agent in &mut self.agents{
            let _ = agent.execute(&mut self.factsheet).await;
            let agent_info = agent.get_attribute_from_agent();
            dbg!(agent_info);
        }
        let api_schema_str = serde_json::to_string(&self.factsheet.api_endpoint_schema)
            .expect("Could not serialize api schema");
        save_api_json(&api_schema_str, "user1");
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[tokio::test]
    async fn test_managing_agent(){
        let usr_req = "need a fullstack app that tracks and fetches my diets , need info about different diets as per the time zone";
        let mut managing_agent = ManagingAgent::new(usr_req.to_string()).await.expect("Managing agent failed");
        let _ = managing_agent.execute_project().await;
        assert!(managing_agent.factsheet.external_urls.is_some());
        dbg!(managing_agent.factsheet);
    }
}