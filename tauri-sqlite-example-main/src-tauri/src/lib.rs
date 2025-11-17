use tauri_plugin_sql::{Migration, MigrationKind};  
mod sysvar;
mod commands;
mod llm_proxy;
use ollama_rs::Ollama;
use tokio::sync::Mutex;
use tauri::State;
//use ollama_rs::chat::{ChatMessage,ChatMessageRequest};
//use futures::StreamExt;
//use tauri::async_runtime::Channel;




#[tauri::command]  
fn greet(name: &str) -> String {  
    format!("Hello, {}! You've been greeted from Rust!", name)  
}

pub struct AppState {  
    pub ollama: Mutex<Ollama>,  
}

#[tauri::command]
async fn get_models(state:State<'_,AppState>) -> Result<Vec<String>,String>{
let models ={
    let client = state.ollama.lock().await;
    client.list_local_models()
    .await
    .map_err(|e|format!("Failed to list models: {:?}",e))?
};

    Ok(models.into_iter().map(|m| m.name.clone()).collect())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]  
pub fn run() {  
    let migrations = vec![  
        Migration {  
            version: 1,  
            description: "create users table",  
            sql: "CREATE TABLE IF NOT EXISTS users (  
                id INTEGER PRIMARY KEY AUTOINCREMENT,  
                name TEXT NOT NULL,  
                email TEXT  
            )",  
            kind: MigrationKind::Up,  
        }  
    ];  

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:test.db", migrations)
                .build()
        )
        .manage(AppState {  ollama: Mutex::new(Ollama::default()) })
        .plugin(tauri_plugin_shell::init())  
        .invoke_handler(tauri::generate_handler![
            greet,
            get_models,
            sysvar::get_keys,
            commands::ollama_chat,
            
            ])  
        .setup(|_| {
                // Optional: print configured OLLAMA_URL
        let cfg = std::env::var("OLLAMA_URL").unwrap_or_else(|_| "http://127.0.0.1:11434".into());
        println!("OLLAMA_URL = {}", cfg);
        Ok(())
    })
        .run(tauri::generate_context!())  
        .expect("error while running Tauri application");  
}
