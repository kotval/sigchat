use std::io::{Error, ErrorKind};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use anyhow::anyhow;
use bytes::Bytes;
use rustls::{ClientConnection, StreamOwned};
use tls::Tls;
use tungstenite::{Error as TunsteniteError, Message, WebSocket};
use url::Url;

use crate::signalservice::{WebSocketMessage, WebSocketRequestMessage, WebSocketResponseMessage};

const PROVISIONING_PATH: [&str; 4] = ["v1", "websocket", "provisioning", ""];
const REGISTRATION_PATH: [&str; 3] = ["v1", "registration", ""];

#[allow(dead_code)]
pub struct SignalWS {
    ws: Arc<Mutex<WebSocket<StreamOwned<ClientConnection, TcpStream>>>>,
}

fn ws_response(msg: tungstenite::Message) -> anyhow::Result<WebSocketResponseMessage> {
    let frame = msg.into_data();
    let ws_msg: WebSocketMessage = prost::Message::decode(Bytes::from(frame))?;
    //TODO:: check that we actually got a response and make nice error
    let response = match ws_msg.response {
        Some(b) => b,
        None => panic!("No Response in message"),
    };
    let response: WebSocketResponseMessage = prost::Message::decode(response.body())?;
    Ok(response)
}

impl SignalWS {
    #[allow(dead_code)]
    pub fn new_message(_host: &str) -> Result<Self, Error> {
        todo!();
    }

    fn new(url: &Url) -> Result<Self, Error> {
        match SignalWS::connect(url) {
            Ok(ws) => Ok(Self { ws: Arc::new(Mutex::new(ws)) }),
            Err(e) => Err(e),
        }
    }

    pub fn new_provision(url: &mut Url) -> Result<Self, Error> {
        url.set_scheme("wss").expect("failed to set scheme");
        url.path_segments_mut().expect("failed to add path").extend(&PROVISIONING_PATH);
        Ok(Self::new(&url)?)
    }

    #[allow(dead_code)]
    pub fn new_register(url: &mut Url) -> Result<Self, Error> {
        url.set_scheme("wss").expect("failed to set scheme");
        url.path_segments_mut().expect("failed to add path").extend(&REGISTRATION_PATH);
        Ok(Self::new(&url)?)
    }

    pub fn close(&mut self) {
        log::info!("attempting to close websocket connection");
        let ws = self.ws.clone();
        thread::spawn(move || {
            loop {
                if let Ok(mut ws) = ws.lock() {
                    ws.close(None).unwrap_or_else(|e| log::warn!("failed to close websocket: {e}"));
                    loop {
                        match ws.flush() {
                            Ok(()) => (),
                            Err(tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed) => {
                                log::info!("websocket connection closed");
                                break;
                            }
                            Err(e) => {
                                log::warn!("{e}");
                                break;
                            }
                        }
                    }
                };
            }
        });
    }

    /// Reads a msg from the websocket with optional timeout
    ///
    /// Hint: timeout = None is more efficient than Some(Duration(1)
    ///
    /// # Arguments
    ///
    /// * `timeout` - a duration before the read operation times-out and returns
    ///
    /// # Returns
    /// a message read from the websocket or ErrorKind::TimedOut
    pub fn read(&mut self, timeout: Option<Duration>) -> anyhow::Result<WebSocketResponseMessage> {
        match timeout {
            Some(duration) => {
                let (tx, rx) = std::sync::mpsc::channel();
                let ws = self.ws.clone();
                thread::spawn(move || {
                    if let Ok(mut ws) = ws.lock() {
                        tx.send(ws.read()).unwrap_or_else(|e| log::warn!("failed to forward ws msg: {e}"));
                    }
                });
                let msg = rx.recv_timeout(duration).unwrap().unwrap();
                //let frame = msg.into_data();
                //let ws_msg: WebSocketMessage = prost::Message::decode(frame.into())?;
                //let response: WebSocketResponseMessage = prost::Message::decode(ws_msg.into())?;
                //Ok(response)
                ws_response(msg)
            }
            None => match self.ws.lock().and_then(|mut ws| Ok(ws.read())) {
                Ok(msg) => ws_response(msg?),
                Err(e) => {
                    log::warn!("Read error: {}", e);
                    Err(anyhow!("locked up oof"))
                }
            },
        }
    }

    #[allow(dead_code)]
    pub fn send(&mut self, _message: Message) -> Result<(), Error> { todo!() }

    /// Make a websocket connection to host server
    ///
    /// # Arguments
    /// * `url` - url of Signal server
    ///
    /// # Returns
    fn connect(url: &Url) -> Result<WebSocket<StreamOwned<ClientConnection, TcpStream>>, Error> {
        log::info!("attempting websocket connection to {}", url.as_str());
        let host = url.host_str().expect("failed to extract host from url");
        match TcpStream::connect((host, 443)) {
            Ok(sock) => {
                log::info!("tcp connected to {host}");
                let xtls = Tls::new();
                match xtls.stream_owned(host, sock) {
                    Ok(tls_stream) => {
                        log::info!("tls configured");
                        match tungstenite::client(url, tls_stream) {
                            Ok((socket, response)) => {
                                log::info!("Websocket connected to: {}", url.as_str());
                                log::info!("Response HTTP code: {}", response.status());
                                Ok(socket)
                            }
                            Err(e) => {
                                log::info!("failed to connect websocket: {}", e);
                                Err(Error::from(ErrorKind::ConnectionRefused))
                            }
                        }
                    }
                    Err(e) => {
                        log::warn!("failed to configure tls: {e}");
                        Err(e)
                    }
                }
            }
            Err(e) => {
                log::warn!("failed to connect tcp: {e}");
                Err(e)
            }
        }
    }
}
