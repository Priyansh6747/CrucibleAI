use crate::models::agent_basic::basic_agent::BasicAgent;
use crate::models::agents::agent_traits::{FactSheet, SpecialFunction};

pub struct ManagingAgent {
    attribute: BasicAgent,
    factsheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunction>>,
}