use tauri::command;
use serde::{Deserialize, Serialize};


use crate::llm_proxy;


#[derive(Deserialize)]
pub struct ChatInput {
pub model: String,
pub prompt: String,
}


#[derive(Serialize)]
pub struct ChatOutput {
pub ok: bool,
pub text: String,
}


#[command]
pub fn ollama_chat(input: ChatInput) -> Result<ChatOutput, String> {
match llm_proxy::call_llm_blocking(&input.model, &input.prompt) {
Ok(reply) => Ok(ChatOutput { ok: true, text: reply }),
Err(e) => Ok(ChatOutput { ok: false, text: format!("LLM error: {}", e) }),
}
}