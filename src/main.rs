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
        // We need to get 5000, if we get only 100 then it's possible to reduce count of levels at the beginning (if our_levels qty just come as 0.0000000)
        let snapshot_string = get_start_snapshot_from("https://data-api.binance.vision/api/v3/depth?symbol=BNBBTC&limit=5000").await.unwrap();
        let snapshot = json_helper::parse_snapshot(&snapshot_string).unwrap();
        let snapshot_id = snapshot.lastUpdateId;

        if snapshot_id >= first_increment_id {
            return snapshot;
        }
    }
    // We couldn't take first snapshot. We hardexit assuming there is nothing to do at all
    panic!()
}

async fn task_for_loop(ws_connection: Option<ws_connector_impl::Connection>, price_levels_shared: std::sync::Arc<std::sync::Mutex<PriceLevels>>, ws_url: &str, no_delay: bool, conn_num: i8) {
    let local_connection = match ws_connection {
        Some(ws_connection) => ws_connection,
        None =>  ws_connector_impl::Connection::make_connection_to(ws_url, no_delay).await.unwrap()
    };

    let mut local_connection_boxed = Box::new(local_connection); 

    loop {
        while let Some(message) = local_connection_boxed.get_message().await {
            let mes = message.unwrap();
            let inc_update = json_helper::parse_incremental(mes.to_text().unwrap());
            match inc_update {
                Ok(inc_update) => {
                    let mut price_levels_ = price_levels_shared.lock().unwrap();
                    let res = price_levels_.update_from_incremental(inc_update, conn_num);
                    match res {
                        Ok(()) => (), // for check: println!("{price_levels_:?}"),
                        Err(_) => ()//TODO reconnect (Somethin went wrong)
                    }
                },
                //Sometimes they sent just one int, so assume it is normal and just skip
                Err(_) => (),
            };
        }   
    }
}


fn main()  {
    tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(async {
        
        let ws_url = "wss://stream.binance.com:9443/ws/bnbbtc@depth@100ms";   
        let mut ws_connection = ws_connector_impl::Connection::make_connection_to(ws_url, false).await.unwrap();

        let message = ws_connection.get_message().await;
        let mes = message.unwrap().unwrap();
        let first_incremental = json_helper::parse_incremental(mes.to_text().unwrap()).unwrap();
        let first_increment_id: i64 = first_incremental.U;

        let snapshot = get_first_snapshot(first_increment_id).await;
        
        let mut price_levels = PriceLevels::make_init_price_levels_from_snapshot(snapshot);
        
        println!("{price_levels:?}");

        price_levels.update_from_incremental(first_incremental, 1).unwrap();

        
        println!("{price_levels:?}");
        let counter1 = std::sync::Arc::new(std::sync::Mutex::new(price_levels));
        let counter2 = std::sync::Arc::clone(&counter1);
        let counter3 = std::sync::Arc::clone(&counter1);
        let counter4 = std::sync::Arc::clone(&counter1);


        let fut1 = async move {
            task_for_loop(Some(ws_connection), counter1, ws_url, false, 1).await;
        };

        let fut2 = async move {
            // We set no_delay in one of connection (may be we are close to server)
            task_for_loop(None, counter2, ws_url, true, 2).await;
        };

        let fut3 = async move {
            task_for_loop(None, counter3, ws_url, false, 3).await;
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