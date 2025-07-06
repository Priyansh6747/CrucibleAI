use serde::{Serialize, Deserialize};
use crate::models::general::llm::GeminiContent;
use crate::models::agent_basic::basic_traits::BasicTrait;
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
pub enum AgentState {
    Discovery,
    Working,
    UnitTesting,
    Finished,
}


#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub struct BasicAgent {
    pub objective: String,
    pub position:String,
    pub state: AgentState,
    pub memory: Vec<GeminiContent>,
}


impl BasicTrait for BasicAgent {
    fn new(objective:String , position:String) -> Self{
        Self {
            objective,
            position,
            state: AgentState::Discovery,
            memory: vec![],
        }
    }

    fn update_state(&mut self, new_state:AgentState) {
        self.state = new_state;
    }
    fn get_objective(&self) -> &String {
        &self.objective
    }
    fn get_position(&self) -> &String {
        &self.position
    }
    fn get_state(&self) -> &AgentState {
        &self.state
    }
    fn get_memory(&self) -> &Vec<GeminiContent> {
        &self.memory
    }
}