use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

use futures_util::stream::SplitSink;

#[derive(Debug)]
pub struct Connection {
    pub id: u32,
    pub sender: SplitSink<WebSocketStream<TcpStream>, Message>,
}

impl Connection {
    pub fn new(id: u32, sender: SplitSink<WebSocketStream<TcpStream>, Message>) -> Self {
        Self { id: id, sender }
    }
}
