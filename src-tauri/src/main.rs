// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use anyhow::anyhow;

fn sample_binary_method(input: Vec<usize>) -> String {
    format!("sum: {}", input.iter().sum::<usize>())
}
fn __mybinary_ipc_sample_binary_method(
    bin: &[u8],
    buf: &mut Vec<u8>,
) -> Result<(), bincode::Error> {
    #[derive(serde::Deserialize)]
    struct Args(Vec<usize>);

    let args: Args = bincode::deserialize(bin)?;

    let ret = sample_binary_method(args.0);

    bincode::serialize_into(buf, &ret)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .register_uri_scheme_protocol("mybinary", |_, req| {
            let method = 'method: {
                let Some(query_part) = req.uri().split_once('?').map(|t| t.1) else {
                    break 'method None;
                };
                query_part.strip_prefix("method=")
            };

            let Some(method) = method else {
                return Err(anyhow!("no method was provided.").into());
            };

            let mut buf = Vec::new();
            match method {
                "sample_binary_method" => {
                    __mybinary_ipc_sample_binary_method(req.body(), &mut buf)?
                }
                m => return Err(anyhow!("unrecognized method. {}", m).into()),
            }

            tauri::http::ResponseBuilder::new()
                .header(tauri::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .body(buf)
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
