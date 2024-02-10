use tauri::async_runtime::{self, JoinHandle};

use crate::protocol::{Demuxer, Port};
use crate::veclist::VecList;
use std::cell::RefCell;
use std::sync::mpsc::{self, Receiver, Sender};

struct HandlerEntry<H: SocketHandler> {
    sender: mpsc::Sender<Vec<u8>>,
    receiver: mpsc::Receiver<Vec<u8>>,
    handle: JoinHandle<Result<(), H::Error>>,
}

struct SocketDemuxer<R: tauri::Runtime, C: SocketConnector<R>> {
    handlers: RefCell<VecList<HandlerEntry<C::Handler>>>,
    connector: C,
}

impl<R: tauri::Runtime, C: SocketConnector<R>> Demuxer<R> for SocketDemuxer<R, C> {
    type Error = C::Error;

    fn connect(
        &self,
        app: tauri::AppHandle<R>,
        event_path: String,
        payload: Vec<u8>,
    ) -> Result<crate::protocol::Port, Self::Error> {
        let handler = self.connector.connect(app, event_path, payload)?;

        let (input_sender, input_receiver) = mpsc::channel();
        let (output_sender, output_receiver) = mpsc::channel();

        let entry = HandlerEntry {
            sender: input_sender,
            receiver: output_receiver,
            handle: async_runtime::spawn(handler.handle(todo!())),
        };

        let port = self.handlers.borrow_mut().append(entry);

        Ok(Port::from_u32(port as u32))
    }

    fn input(&self, port: crate::protocol::Port, payload: Vec<u8>) -> Result<(), Self::Error> {
        let binding = self.handlers.borrow();
        let Some(entry) = binding.get(port.as_u32() as usize) else {
            return Err(todo!());
        };

        match entry.sender.send(payload) {
            Ok(_) => Ok(()),
            Err(e) => Err(todo!()),
        }
    }

    fn output(&self, port: Port) -> Result<Vec<u8>, Self::Error> {
        let binding = self.handlers.borrow();
        let Some(entry) = binding.get(port.as_u32() as usize) else {
            return Err(todo!());
        };

        let payload = match entry.receiver.recv() {
            Ok(v) => v,
            Err(e) => return Err(todo!()),
        };

        Ok(payload)
    }

    fn error(&self, port: crate::protocol::Port, payload: Vec<u8>) -> Result<(), Self::Error> {
        todo!()
    }

    fn close(&self, port: crate::protocol::Port, payload: Vec<u8>) -> Result<(), Self::Error> {
        todo!()
    }
}

trait SocketConnector<R: tauri::Runtime> {
    type Handler: SocketHandler;
    type Error: std::error::Error;
    fn connect(
        &self,
        app: tauri::AppHandle<R>,
        event_path: String,
        payload: Vec<u8>,
    ) -> Result<Self::Handler, Self::Error>;
}

trait SocketHandler {
    type Error: Send + std::error::Error;
    type Future: Send + std::future::Future<Output = Result<(), Self::Error>>;
    fn handle(&self, socket: Socket) -> Self::Future;
}

struct Socket {
    sender: Sender<Vec<u8>>,
    receiver: Receiver<Vec<u8>>,
}

impl<E, F, Fun> SocketHandler for Fun
where
    E: Send + std::error::Error,
    F: Send + std::future::Future<Output = Result<(), E>>,
    Fun: Fn(Socket) -> F,
{
    type Error = E;
    type Future = F;

    fn handle(&self, socket: Socket) -> Self::Future {
        self(socket)
    }
}
