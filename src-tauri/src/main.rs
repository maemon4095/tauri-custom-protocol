// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::anyhow;

// connection handshake

// カスタムプロトコルごとにサーバ，クライアントを作る．ディスパッチャは用意しない方針にする．
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .register_uri_scheme_protocol("bin_channel", |app, req| {
            let headers = req.headers();

            match headers.get("X-Tauri-Bin-Ipc-Method") {
                Some(method) => {
                    //app内にdispatcherを持たせるのではなく，closureでキャプチャする．
                    match method.as_bytes() {
                        b"CONNECT" => {
                            // handlerを生成し，失敗したらERRORレスポンス．
                            // tokenを生成し，tokenをkeyにhandlerをdispatcherに挿入．
                            // tokenを返す．
                        }
                        b"ERROR" => {}
                        b"DISCONNECT" => {}
                        _ => return Err(anyhow!("bin_channel unknown method.").into()),
                    }
                }
                None => {
                    // methodがなければ接続のレスポンスとする．
                    let Some(token) = headers.get("X-Tauri-Bin-Ipc-Token") else {
                        return Err(anyhow!("bin_channel missing token.").into());
                    };
                }
            }

            tauri::http::ResponseBuilder::new()
                .header("Access-Control-Allow-Origin", "*")
                .body(Vec::new())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
