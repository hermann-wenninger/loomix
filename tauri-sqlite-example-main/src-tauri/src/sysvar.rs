use std::env;
use tauri::command;
use serde_json;
use once_cell::sync::Lazy;

pub static GROQ_KEY: Lazy<String> = Lazy::new(|| {env::var("GROQ_API_KEY").unwrap_or_default()});
pub static MISTRAL_KEY:Lazy<String> = Lazy::new(|| {env::var("MISTRAL_API_KEY").unwrap_or_default()});
pub static OLLAMA_KEY: Lazy<String> = Lazy::new(|| {env::var("OLLAMA_API_KEY").unwrap_or_default()});
pub static OPENAI_KEY: Lazy<String> = Lazy::new(|| {env::var("OPENAI_API_KEY").unwrap_or_default()});
pub static OPENROUTER: Lazy<String> = Lazy::new(|| {env::var("OPENROUTER_API_KEY").unwrap_or_default()});
pub static ANTROPIC_KEY: Lazy<String> = Lazy::new(|| {env::var("ANTHROPIC_API_KEY").unwrap_or_default()});
pub static COHERCE_KEY:Lazy<String> = Lazy::new(|| {env::var("COHERE_API_KEY").unwrap_or_default()});
pub static DEEPSEEK_KEY:Lazy<String> = Lazy::new(|| {env::var("DEEPSEEK_API_KEY").unwrap_or_default()});
pub static GOOGLE_KEY: Lazy<String> = Lazy::new(|| {env::var("GOOGLE_API_KEY").unwrap_or_default()});
pub static IIELEVEN_KEY: Lazy<String> = Lazy::new(|| {env::var("IIELEVENLABS_API_KEY").unwrap_or_default()});
pub static SERPAPI_KEY: Lazy<String> = Lazy::new(|| {env::var("SERPAPI_API_KEY").unwrap_or_default()});


#[command]
pub fn get_keys() -> serde_json::Value {
serde_json::json!({
    "GROQ_KEY": *GROQ_KEY,
    "MISTRAL_KEY": *MISTRAL_KEY,
    "OLLAMA_KEY": *OLLAMA_KEY,
    "OPENAI_KEY": *OPENAI_KEY,
    "OPENROUTER": *OPENROUTER,
    "ANTROPIC_KEY": *ANTROPIC_KEY,
    "COHERCE_KEY": *COHERCE_KEY,
    "DEEPSEEK_KEY": *DEEPSEEK_KEY,
    "GOOGLE_KEY": *GOOGLE_KEY,
    "IIELEVEN_KEY": *IIELEVEN_KEY,
    "SERPAPI_KEY": *SERPAPI_KEY,
})
    }
  
