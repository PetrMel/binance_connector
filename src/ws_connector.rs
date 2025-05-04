pub mod ws_connector_impl {






use std::io::{stdout, Write};

use futures_util::{StreamExt, TryStreamExt};
use reqwest::{Request, StatusCode};
use tokio_tungstenite::connect_async;
use tungstenite::{client::IntoClientRequest, Message};

use tokio_tungstenite::MaybeTlsStream;



pub struct Connection {
   // stream : tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>
}


impl Connection {
    pub async fn make_connection_to(url : &str) { //-> Result<Connection, eyre::Error> {
        let request = url.into_client_request().unwrap();
        
        let (stream_local, response) = connect_async(request).await.unwrap();

        let status_code = response.status();
        match status_code {
            StatusCode::SWITCHING_PROTOCOLS => println!("ws_connected"),
            _ => panic!("expected response 101, but code is: {status_code}"),
        }
        let fut = stream_local.for_each(|message| async {
            let data =  message.unwrap().into_data();
            stdout().write(&data).unwrap();
            stdout().flush();
        });

        fut.await;
        // let read_future = stream_local.for_each(|message| async {
        //     let data = message.unwrap().into_data();
        //     stdout().write(&data).await.unwrap();
        //     stdout().write(b"\n").await.unwrap();
        //     stdout().flush().await.unwrap();
        // });

        //read_future.await;        

        //Ok(Self { stream: (stream_local) })
    }


    pub async fn get_message(self)  {
        // let read_future = self.stream.for_each(|message| async {
        //     let data = message.unwrap().into_data();
        //     tokio::io::stdout().write(&data).await.unwrap();
        //     tokio::io::stdout().write(b"\n").await.unwrap();
        //     tokio::io::stdout().flush().await.unwrap();
        // });

        // read_future.await;
    }

    
}

}