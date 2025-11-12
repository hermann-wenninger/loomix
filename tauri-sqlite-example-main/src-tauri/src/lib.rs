use tauri_plugin_sql::{Migration, MigrationKind};  
mod sysvar;
use ollama-rs::Ollama;
use tokio::sync::Mutex;

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
}

    Ok(models.iter().map(|m| m.name.clone()).collect())
}

async fn chat(state:State<'_,AppState>,request:ChatRequest, on_stream:Channel<ChatResponse>) -> Result<(),String>{
   
        let mut client = state.ollama.lock().await;
        let chat_request = ChatMessageRequest::new(request.model,request.messages);
        let mut stream = client.send_chat_mesages_stream(chat_request)
        .await
        .map_err(|e|format!("Chat stream failed: {:?}",e))?;
        
        while let Some(response) = stream.next().await {
            let response = response.map_err(|e| format!("Stream error: {:?}",e))?;
            let chat_response = ChatResponse{
                message:response.message.content,
            };
            on_stream.send(chat_response).await.map_err(|e|format!("Failed to send response: {:?}",e))?;
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
            sysvar::get_keys,
            
            ])  
        .run(tauri::generate_context!())  
        .expect("error while running Tauri application");  
}
