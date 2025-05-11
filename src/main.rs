mod http_connector;
mod json_parser;
mod ws_connector;
mod price_levels_engine;

pub use http_connector::get_request;
use http_connector::get_request::get_start_snapshot_from;
pub use json_parser::json_helper;
use json_parser::json_helper::PriceLevelsSnapshot;
pub use ws_connector::ws_connector_impl;
pub use price_levels_engine::price_levels_engine::PriceLevels;

async fn get_first_snapshot(first_increment_id : i64) -> PriceLevelsSnapshot {
    for i in 1..6 {
        print!("attempt {i}");
        let snapshot_string = get_start_snapshot_from("https://data-api.binance.vision/api/v3/depth?symbol=BNBBTC&limit=100").await.unwrap();
        let snapshot = json_helper::parse_snapshot(&snapshot_string).unwrap();
        let snapshot_id = snapshot.lastUpdateId;

        if snapshot_id >= first_increment_id {
            return snapshot;
        }
    }
    // We couldn't take first snapshot. We hardexit assuming there is nothing to do at all
    panic!()
}

fn main()  {
    tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(async {
        
        let ws_url = "wss://stream.binance.com:9443/ws/bnbbtc@depth@100ms";   
        let mut ws_connection = ws_connector_impl::Connection::make_connection_to(ws_url).await.unwrap();

        let message = ws_connection.get_message().await;
        let mes = message.unwrap().unwrap();
        let first_incremental = json_helper::parse_incremental(mes.to_text().unwrap()).unwrap();
        let first_increment_id: i64 = first_incremental.U;

        let snapshot = get_first_snapshot(first_increment_id).await;
        
        let mut price_levels = PriceLevels::make_init_price_levels_from_snapshot(snapshot);
        
        println!("{price_levels:?}");

        price_levels.update_from_incremental(first_incremental, 1);

        
        println!("{price_levels:?}");
        let counter1 = std::sync::Arc::new(std::sync::Mutex::new(price_levels));
        let counter2 = std::sync::Arc::clone(&counter1);
        let counter3 = std::sync::Arc::clone(&counter1);
        let counter4 = std::sync::Arc::clone(&counter1);


        let fut1 = async move {
            while let Some(message) = ws_connection.get_message().await {
                let mes = message.unwrap();
                let inc_update = json_helper::parse_incremental(mes.to_text().unwrap());
                match inc_update {
                    Ok(inc_update) => {
                        let mut price_levels_ = counter1.lock().unwrap();
                        price_levels_.update_from_incremental(inc_update, 1);
                        println!("{price_levels_:?}");
                    },
                    Err(_) => (),
                                    };
            }
        };

        let fut2 = async move {
            let mut ws_connection_local = ws_connector_impl::Connection::make_connection_to(ws_url).await.unwrap();
            while let Some(message) = ws_connection_local.get_message().await {
                let mes = message.unwrap();
                let inc_update = json_helper::parse_incremental(mes.to_text().unwrap());
                match inc_update {
                    Ok(inc_update) => {
                        let mut price_levels_ = counter2.lock().unwrap();
                        price_levels_.update_from_incremental(inc_update, 2);
                        println!("{price_levels_:?}");
                    },
                    Err(_) => (),
                                    };
            }
        };

        let fut3 = async move {
            let mut ws_connection_local = ws_connector_impl::Connection::make_connection_to(ws_url).await.unwrap();
            while let Some(message) = ws_connection_local.get_message().await {
                let mes = message.unwrap();
                let inc_update = json_helper::parse_incremental(mes.to_text().unwrap());
                match inc_update {
                    Ok(inc_update) => {
                        let mut price_levels_ = counter3.lock().unwrap();
                        price_levels_.update_from_incremental(inc_update, 3);
                        println!("{price_levels_:?}");
                    },
                    Err(_) => (),
                                    };
            }
        };

        let fut4 = async move {
                loop {
                    {
                        let mut price_levels_ = counter4.lock().unwrap();
                        let json_text = price_levels_.as_json_text();
                        println!("{}", json_text);
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
        };



        let handle1 = tokio::spawn(fut1);
        let handle2 = tokio::spawn(fut2);
        let handle3 = tokio::spawn(fut3);
        let handle4 = tokio::spawn(fut4);

        let res = tokio::join!(handle1, handle2, handle3, handle4);
        res.0.unwrap();

    });
}