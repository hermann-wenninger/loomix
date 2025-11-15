use tauri_plugin_sql::{Migration, MigrationKind};  
mod sysvar;
use ollama_rs::Ollama;
use tokio::sync::Mutex;
use serde::Deserialize;
use serde::Serialize;
use tauri::State;
//use ollama_rs::chat::{ChatMessage,ChatMessageRequest};
//use futures::StreamExt;
//use tauri::async_runtime::Channel;
use tauri::ipc::Channel;
use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::generation::chat::ChatMessage;
use futures_util::StreamExt;



#[tauri::command]  
fn greet(name: &str) -> String {  
    format!("Hello, {}! You've been greeted from Rust!", name)  
}

pub struct AppState {  
    pub ollama: Mutex<Ollama>,  
}
#[derive(Serialize)]
struct ChatResponse{
    message:String,
}
#[derive(Deserialize)]
struct ChatRequest{
    model:String,
    messages:Vec<ChatMessage>,
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

#[tauri::command]
async fn chat(state:State<'_,AppState>,request:ChatRequest, on_stream:Channel<ChatResponse>) -> Result<(),String>{
   
        let client = state.ollama.lock().await;
        let chat_request = ChatMessageRequest::new(request.model,request.messages);
        let mut stream = client.send_chat_messages_stream(chat_request)
        .await
        .map_err(|e|format!("Chat stream failed: {:?}",e))?;
        
        while let Some(response) = stream.next().await {
            let response = response.map_err(|e| format!("Stream error: {:?}",e))?;
            let chat_response = ChatResponse{
                message:response.message.content,
            };
            on_stream.send(chat_response).map_err(|e|format!("Failed to send response: {:?}",e))?;
}
    Ok(())
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
            chat,
            sysvar::get_keys,
            
            ])  
        .run(tauri::generate_context!())  
        .expect("error while running Tauri application");  
}
