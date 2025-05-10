mod http_connector;
mod json_parser;
mod ws_connector;

pub use http_connector::get_request;
use http_connector::get_request::get_start_snapshot_from;
pub use json_parser::json_helper;
use json_parser::json_helper::PriceLevelsSnapshot;
pub use ws_connector::ws_connector_impl;


#[derive(Debug)]
pub struct PriceLevels {
    last_update_id: i64,
    bids: std::collections::BTreeMap<String, String>,
    asks: std::collections::BTreeMap<String, String>
}

impl PriceLevels {

    fn update_from_incremental (&mut self, inc_upd: json_helper::PriceLevelsIncremental) {

        if (inc_upd.u < self.last_update_id) {
            // Nothing to do
            return;
        }
        let id: thread::ThreadId = thread::current().id();
        println!("{id:?} : {inc_upd:?}");

        if (inc_upd.U > self.last_update_id+1) {
            panic!("Something went wrong");
        }

        self.last_update_id = inc_upd.u;
        
        //TODO make closure
        for ask in inc_upd.a {
            let parsed: f32 = ask.1.parse().unwrap();
            //TODO comp double with 0
            if parsed == 0.0 {
                self.asks.remove(ask.0.to_string().as_str());
                continue;
            }

            self.asks.insert(ask.0, ask.1);
        }

        for bid in inc_upd.b {
            let parsed: f32 = bid.1.parse().unwrap();
            //TODO comp double with 0
            if parsed == 0.0 {
                self.bids.remove(bid.0.to_string().as_str());
                continue;
            }

            self.bids.insert(bid.0, bid.1);
        }


    }


}


use std::{collections::BTreeMap, io::Write, sync::Arc, thread};

async fn get_first_snapshot(first_increment_id : i64) -> PriceLevelsSnapshot {
    for i in 1..6 {
        print!("attempt {i}");
        let snapshot_string = get_start_snapshot_from("https://data-api.binance.vision/api/v3/depth?symbol=BNBBTC&limit=100").await.unwrap();
        let snapshot = json_helper::parse_snapshot(&snapshot_string).unwrap();
        let snapshot_id = snapshot.lastUpdateId;

        if (snapshot_id >= first_increment_id) {
            return snapshot;
        }
    }
    panic!()
}

fn main()  {
    tokio::runtime::Builder::new_current_thread()
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
        let bids1 = BTreeMap::from_iter(snapshot.bids);
        let asks1 = BTreeMap::from_iter(snapshot.asks);


        let mut price_levels : PriceLevels  = PriceLevels{last_update_id: snapshot.lastUpdateId, bids: bids1, asks : asks1};
        
        

        println!("{price_levels:?}");

        price_levels.update_from_incremental(first_incremental);

        
        println!("{price_levels:?}");
        let mtx_local = std::sync::Arc::new(std::sync::Mutex::new(price_levels));

        let counter1 = Arc::clone(&mtx_local);


        let fut1 = async move {
            while let Some(message) = ws_connection.get_message().await {
                let mes = message.unwrap();
                let inc_update = json_helper::parse_incremental(mes.to_text().unwrap());
                match inc_update {
                    Ok(inc_update) => {
                        let mut price_levels_ = mtx_local.lock().unwrap();
                        price_levels_.update_from_incremental(inc_update);
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
                        let mut price_levels_ = counter1.lock().unwrap();
                        price_levels_.update_from_incremental(inc_update);
                        println!("{price_levels_:?}");
                    },
                    Err(_) => (),
                                    };
            }
        };

        let fut11 = tokio::spawn(fut1);
        let fut22 = tokio::spawn(fut2);

        tokio::join!(fut11,fut22);

    });
}