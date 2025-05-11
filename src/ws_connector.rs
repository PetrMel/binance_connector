pub mod ws_connector_impl {

use futures_util::StreamExt;
use reqwest::StatusCode;
use tokio_tungstenite::connect_async;

use tokio_tungstenite::MaybeTlsStream;
use tokio::net::TcpStream;


pub struct Connection {
   stream : tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>
}


impl Connection {
    pub async fn make_connection_to(url : &str) -> Result<Connection, eyre::Error> {
        let request = tokio_tungstenite::tungstenite::client::IntoClientRequest::into_client_request(url).unwrap();
        
        let (stream_local, response) = connect_async(request).await.unwrap();

        let status_code = response.status();
        match status_code {
            StatusCode::SWITCHING_PROTOCOLS => println!("ws_connected"),
            _ => panic!("expected response 101, but code is: {status_code}"),
        }


        Ok(Self { stream: (stream_local) })
    }



    pub async fn get_message(&mut self) -> Option<Result<tokio_tungstenite::tungstenite::Message, tokio_tungstenite::tungstenite::Error>> {
        let message = self.stream.next().await;
        message
    }

    
}

}