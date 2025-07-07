use std::fs;
use reqwest::Client;
use serde::de::DeserializeOwned;
use crate::api::call_req::call_api;
use crate::helper::command_line::PrinCommand;
use crate::models::general::llm::{GeminiContent, GeminiResponse};

const CODE_TEMPLATE_PATH: &str = "../../../Template/ServerTemplate.rs";

pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> GeminiContent {
    let ai_function_str: &str = ai_func(func_input);

    // Extend the string to encourage only printing the output
    let msg: String = format!(
        "FUNCTION: {}
  INSTRUCTION: You are a function printer. You ONLY print the results of functions.
  Nothing else. No commentary. Here is the input to the function: {}.
  Print out what the function will return.",
        ai_function_str, func_input
    );

    GeminiContent::new("user".to_string(), msg)
}

pub async fn ai_task_req(
    msg_ctx: String,
    agent_pos: &str,
    agent_operation: &str,
    fn_pass: fn(&str) -> &'static str
) -> GeminiResponse {
    let extended_msg = extend_ai_function(fn_pass, &msg_ctx);
    PrinCommand::AICall.print_agent_message(agent_pos, agent_operation);
    let llm_res: Result<GeminiResponse, Box<dyn std::error::Error + Send + Sync>> = call_api(vec![extended_msg.clone()]).await;
    match llm_res {
        Ok(llm_res) => llm_res,
        Err(_) => call_api(vec![extended_msg.clone()])
            .await
            .expect("Failed to connect to LLM")
    }
}

pub async fn ai_task_req_decoded<T: DeserializeOwned>(
    msg_ctx: String,
    agent_pos: &str,
    agent_operation: &str,
    fn_pass: fn(&str) -> &'static str
) -> T {
    let response = ai_task_req(msg_ctx, agent_pos, agent_operation, fn_pass).await;
    let Some(ai_res) = response.get_string() else {
        panic!("LLM gave no text");
    };

    dbg!(ai_res.clone());

    let decoded_res: Result<T, serde_json::Error> = serde_json::from_str(&ai_res);
    let res = match decoded_res {
        Ok(res) => res,
        Err(_) => panic!("Failed to deserialize LLM response")
    };
    res
}

pub async fn check_status_code(client:&Client,url:&str) -> Result<u16,reqwest::Error> {
    let res:reqwest::Response = client.get(url).send().await?;
    Ok(res.status().as_u16())
}

///Get Code Template
pub fn read_code_template_contents() -> String {
    let path = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(&path).expect("Failed to read template file")
}

///Save the new Code
pub fn write_code_main(content: &String , user: &String) {
    let exec_main_path: &str =  &format!("../../../Out/{}/main.rs", user);

    fs::write(&exec_main_path, content).expect("Failed to write main file");

}

///Save the JSON API Endpoint Schema
pub fn save_api_json(api_endpoints:&String , user:&String) {
    let api_schema_path:&str = &format!("../../../Out/{}/api_schema.json", user);
    fs::write(&api_schema_path, api_endpoints).expect("Failed to write api endpoints file");
}
