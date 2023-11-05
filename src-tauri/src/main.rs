// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod demuxer;
mod protocol;
use anyhow::anyhow;
use tauri::http::header::HeaderValue;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .register_uri_scheme_protocol(
            "bin_channel",
            generate_binary_socket_protocol_handler(todo!()),
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
