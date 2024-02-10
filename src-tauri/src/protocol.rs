use anyhow::anyhow;
use tauri::http::header::HeaderValue;
pub struct Port(u32);

impl Port {
    pub fn as_u32(&self) -> u32 {
        self.0
    }

    pub fn from_u32(port: u32) -> Self {
        Self(port)
    }
}

impl TryFrom<Port> for HeaderValue {
    type Error = <HeaderValue as TryFrom<u32>>::Error;

    fn try_from(value: Port) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}
impl<'a> TryFrom<&'a HeaderValue> for Port {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &'a HeaderValue) -> Result<Self, Self::Error> {
        let str = value.to_str().map_err(Into::<Self::Error>::into)?;
        let port = str.parse().map_err(Into::<Self::Error>::into)?;
        Ok(Port(port))
    }
}

pub trait Demuxer<R>
where
    R: tauri::Runtime,
{
    type Error: std::error::Error;

    fn connect(
        &self,
        app: tauri::AppHandle<R>,
        event_path: String,
        payload: Vec<u8>,
    ) -> Result<Port, Self::Error>;
    fn input(&self, port: Port, payload: Vec<u8>) -> Result<(), Self::Error>;
    fn output(&self, port: Port) -> Result<Vec<u8>, Self::Error>;
    fn error(&self, port: Port, payload: Vec<u8>) -> Result<(), Self::Error>;
    fn close(&self, port: Port, payload: Vec<u8>) -> Result<(), Self::Error>;
}

pub fn generate_binary_socket_protocol_handler<R, D>(
    demuxer: D,
) -> impl Fn(
    &tauri::AppHandle<R>,
    &tauri::http::Request,
) -> Result<tauri::http::Response, Box<dyn std::error::Error>>
       + Send
       + Sync
       + 'static
where
    R: tauri::Runtime,
    D: Demuxer<R> + Send + Sync + 'static,
{
    move |app, req| {
        let headers = req.headers();

        const HEADER_CONNECTION_EVENT: &'static str = "X-Tauri-Bin-Ipc-Connection-Event";
        const HEADER_EVENT_PATH: &'static str = "X-Tauri-Bin-Ipc-Event-Path";
        const HEADER_PORT: &'static str = "X-Tauri-Bin-Ipc-Port";

        let res = tauri::http::ResponseBuilder::new().header("Access-Control-Allow-Origin", "*");

        return match headers.get(HEADER_CONNECTION_EVENT) {
            Some(event) => match event.as_bytes() {
                b"CONNECT" => {
                    let Some(path) = headers.get(HEADER_EVENT_PATH) else {
                        return Err(anyhow!("no event path were given.").into());
                    };

                    match demuxer.connect(
                        app.clone(),
                        path.to_str().unwrap().to_string(),
                        req.body().clone(),
                    ) {
                        Ok(port) => res.status(200).header(HEADER_PORT, port).body(Vec::new()),
                        Err(e) => Err(e.into()),
                    }
                }
                b"ERROR" => {
                    // 異常によるソケットの切断
                    let Some(port) = headers.get(HEADER_PORT) else {
                        return Err(anyhow!("bin_channel missing port.").into());
                    };

                    let port: Port = port.try_into()?;

                    match demuxer.error(port, req.body().clone()) {
                        Ok(_) => res.status(200).body(Vec::new()),
                        Err(e) => Err(e.into()),
                    }
                }
                b"CLOSE" => {
                    let Some(port) = headers.get(HEADER_PORT) else {
                        return Err(anyhow!("bin_channel missing port.").into());
                    };

                    let port: Port = port.try_into()?;

                    match demuxer.close(port, req.body().clone()) {
                        Ok(_) => res.status(200).body(Vec::new()),
                        Err(e) => Err(e.into()),
                    }
                }
                _ => Err(anyhow!("bin_channel unknown method.").into()),
            },
            None => {
                // methodが無ければ確立されたソケットのパケットとみなす．
                let Some(port) = headers.get(HEADER_PORT) else {
                    return Err(anyhow!("bin_channel missing port.").into());
                };

                let port: Port = port.try_into()?;

                match req.method().as_str() {
                    "POST" => match demuxer.input(port, req.body().clone()) {
                        Ok(()) => res.status(200).body(Vec::new()),
                        Err(e) => Err(e.into()),
                    },
                    "GET" => match demuxer.output(port) {
                        Ok(payload) => res.status(200).body(payload),
                        Err(e) => Err(e.into()),
                    },
                    _ => {
                        return Err(anyhow!("bin_channel unrecognized method.").into());
                    }
                }
            }
        };
    }
}
