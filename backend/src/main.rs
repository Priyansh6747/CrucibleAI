#[macro_export]
macro_rules! get_function_string {
    ($func: ident) => {{
        stringify!($func)
    }};
}
#[macro_use]
mod ai_functions;
mod api;
mod helper;
mod models;

use crate::models::agent_manager::managing_agent::ManagingAgent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = helper::command_line::get_user_response("What are we building today");
    let agent = ManagingAgent::new(res).await;
    match agent {
        Ok(mut managing_agent) => {
            managing_agent.execute_project().await;
        }
        Err(_) => {
            panic!("Failed to execute project");
        }
    }
    Ok(())
}

