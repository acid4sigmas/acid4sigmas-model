use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::Error;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;

pub struct WsClient {
    write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    receiver: mpsc::UnboundedReceiver<Message>,
}

impl WsClient {
    pub async fn new(url: &str) -> Result<Arc<Mutex<Self>>, Error> {
        let url = Url::parse(url).unwrap();

        let (ws_stream, _) = connect_async(&url).await?;

        let (write, mut read) = ws_stream.split();

        let (tx, rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            while let Some(Ok(message)) = read.next().await {
                if let Err(_) = tx.send(message) {
                    break;
                }
            }
        });

        Ok(Arc::new(Mutex::new(Self {
            write,
            receiver: rx,
        })))
    }

    pub async fn send(&mut self, msg: &str) -> Result<(), tokio_tungstenite::tungstenite::Error> {
        self.write.send(Message::Text(msg.to_string())).await
    }

    pub async fn send_ping(&mut self) -> Result<(), tokio_tungstenite::tungstenite::Error> {
        self.write.send(Message::Ping(Vec::new())).await
    }

    pub async fn receive(&mut self) -> Option<Message> {
        self.receiver.recv().await.or_else(|| None)
    }

    pub async fn reconnect(&mut self, url: &str) -> Result<(), Error> {
        let url = Url::parse(url).unwrap();

        let (ws_stream, _) = connect_async(&url).await?;

        let (write, mut read) = ws_stream.split();

        self.write = write;

        let (tx, rx) = mpsc::unbounded_channel();
        self.receiver = rx;

        tokio::spawn(async move {
            while let Some(Ok(message)) = read.next().await {
                if let Err(_) = tx.send(message) {
                    break;
                }
            }
        });

        Ok(())
    }
}
