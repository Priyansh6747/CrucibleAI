mod ai_functions;
mod api;
mod helper;
mod models;
use crate::models::general::llm::{GeminiContent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = helper::command_line::get_user_response("What are we building today");
    let msg = vec![GeminiContent::new("user".to_string(),res)];

    // Call the API and handle the result
    let out = api::call_req::call_api(msg).await?;
    for s in out.extract_all_texts() {
        println!("{} ", s);
    }
    Ok(())
}

