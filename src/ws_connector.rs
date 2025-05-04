pub mod ws_connect_name {





use reqwest::StatusCode;
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::connect_async;
use tungstenite::client::IntoClientRequest;
use futures_util::StreamExt;

#[tokio::main]
pub async fn ws_connect() {
    let url = String::from("wss://stream.binance.com:9443/ws/bnbbtc@depth@100ms");
    let request = url.into_client_request().unwrap();
    let (stream, response) = connect_async(request).await.unwrap();
    
    let status_code = response.status();
    match status_code {
        StatusCode::SWITCHING_PROTOCOLS => println!("ws_connected"),
        _ => panic!("expected response 101, but code is: {status_code}"),
    }

    let read_future = stream.for_each(|message| async {
        let data = message.unwrap().into_data();
        tokio::io::stdout().write(&data).await.unwrap();
        tokio::io::stdout().write(b"\n").await.unwrap();
        tokio::io::stdout().flush().await.unwrap();
    });

    read_future.await;

}

}