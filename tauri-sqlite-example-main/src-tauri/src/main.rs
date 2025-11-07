// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod actix_server;
use std::thread;


fn main() {
      thread::spawn(|| {
        tauri::async_runtime::block_on(async {
            if let Err(e) = actix_server::start_server().await {
                eprintln!("Actix server failed: {}", e);
            }
        });
    });
    tauri_sqlite_lib::run()
}
