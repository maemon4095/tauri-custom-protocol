struct SocketDemuxer<R: tauri::Runtime, C: SocketConnector<R>> {
    free: Option<usize>,
    ports: Vec<Slot<C::Handler>>,
    connector: C,
}
enum Slot<H: SocketHandler> {
    Free(usize),
    Handler(H),
}

trait SocketConnector<R: tauri::Runtime> {
    type Handler: SocketHandler;
    fn connect(&self, app_handle: tauri::AppHandle<R>, payload: Vec<u8>) -> Self::Handler;
}

trait SocketHandler {
    type Error: std::error::Error;
    type Future: std::future::IntoFuture<Output = Result<(), Self::Error>>;
    fn handle(&self, socket: Socket) -> Self::Future;
}

struct Socket {}

impl<E, F, Fun> SocketHandler for Fun
where
    E: std::error::Error,
    F: std::future::IntoFuture<Output = Result<(), E>>,
    Fun: Fn(Socket) -> F,
{
    type Error = E;
    type Future = F;

    fn handle(&self, socket: Socket) -> Self::Future {
        self(socket)
    }
}
