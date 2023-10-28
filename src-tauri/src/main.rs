// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .register_uri_scheme_protocol("mybinary", |app, req| {
            tauri::async_runtime::block_on(async {
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                let now = chrono::Local::now();
                let now = format!("Request received at: {}", now.to_rfc3339());

                tauri::http::ResponseBuilder::new()
                    .header("Access-Control-Allow-Origin", "*")
                    .body(now.as_bytes().to_vec())
            })
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
