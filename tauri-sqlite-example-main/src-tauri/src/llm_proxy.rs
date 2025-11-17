use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;
use std::env;


#[derive(Serialize, Deserialize, Debug)]
pub struct LlmRequest {
pub model: String,
pub prompt: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct LlmResponse {
pub reply: String,
}


pub fn call_llm_blocking(model: &str, prompt: &str) -> Result<String, String> {
// Endpoint configurable via env var OLLAMA_URL (default local Ollama)
let base = env::var("OLLAMA_URL").unwrap_or_else(|_| "http://127.0.0.1:11434".into());
// Example: user should adapt path according to their LLM HTTP API
let url = format!("{}/api/chat?model={}", base.trim_end_matches('/'), model);


let client = Client::builder().timeout(std::time::Duration::from_secs(30)).build().map_err(|e| e.to_string())?;


// Try a JSON POST - adapt payload to your model server
let payload = serde_json::json!({"model": model, "prompt": prompt});


let resp = client.post(&url).json(&payload).send().map_err(|e| format!("Request error: {}", e))?;


if !resp.status().is_success() {
return Err(format!("LLM returned HTTP {}", resp.status()));
}


// Try to parse JSON with a `reply` field, otherwise return raw text
let text = resp.text().map_err(|e| e.to_string())?;


if let Ok(parsed)| Result::<LlmResponse, _> = serde_json::from_str(&text) {
Ok(parsed.reply)
} else {
// return raw body as best-effort
Ok(text)
}
}