use reqwest::Client;
use crate::api::call_req::call_api;
use crate::helper::command_line::PrinCommand;
use crate::models::general::llm::{GeminiContent, GeminiResponse};

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
    fn_pass: for<'a> fn(&'a str) -> &'static str
) -> GeminiResponse {
    let extended_msg = extend_ai_function(fn_pass, &msg_ctx);
    PrinCommand::AICall.print_agent_message(agent_pos, agent_operation);
    let llm_res: Result<GeminiResponse, Box<dyn std::error::Error>> = call_api(vec![extended_msg.clone()]).await;
    match llm_res {
        Ok(llm_res) => llm_res,
        Err(_) => call_api(vec![extended_msg.clone()])
            .await
            .expect("Failed to connect to LLM")
    }
}

pub async fn check_status_code(client:&Client,url:&str) -> Result<u16,reqwest::Error> {
    let res:reqwest::Response = client.get(url).send().await?;
    Ok(res.status().as_u16())
}


#[cfg(test)]
mod tests {
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;
    use super::*;
    use crate::helper::general::ai_task_req;

    #[tokio::test]
    async fn test_asi_task_req() {
        let ai_func_param = "Build me a webserver for making stock api request".to_string();

        let res = ai_task_req(ai_func_param, "Managing agent", "Define User requirment", convert_user_input_to_goal).await;
        for s in res.extract_all_texts() {
            println!("{} ", s);
        }
        assert!(res.extract_all_texts().len() > 0);
    }
    
    #[tokio::test]
    async fn test_check_status_code() {
        let code = check_status_code(&Client::new(), "https://catfact.ninja/fact").await.unwrap();
        assert_eq!(code, 200u16);
    }
}