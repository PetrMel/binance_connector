mod http_connector;
mod json_parser;
mod ws_connector;

pub use http_connector::get_request;
use http_connector::get_request::get_start_snapshot_from;
pub use json_parser::json_helper;
pub use ws_connector::ws_connector_impl;



pub struct PriceLevels {
    last_update_id: i64,
    bids: std::collections::BTreeMap<String, String>,
    asks: std::collections::BTreeMap<String, String>
}

use std::{collections::BTreeMap, io::{stdout, Write}};

fn main()  {
    trpl::run(async {


        let ws_url = "wss://stream.binance.com:9443/ws/bnbbtc@depth@100ms";   
        let ws_connection = ws_connector_impl::Connection::make_connection_to(&ws_url).await.unwrap();

        let message = ws_connection.get_message().await;
      //  let data = message.into_data();
        let first_incremental = json_helper::parse_incremental(message.to_text().unwrap()).unwrap();
        let first_increment_id: i64 = first_incremental.U;

        let snapshot_string = get_start_snapshot_from("https://data-api.binance.vision/api/v3/depth?symbol=BNBBTC&limit=100").await.unwrap();
        let snapshot = json_helper::parse_snapshot(&snapshot_string).unwrap();
        let snapshot_id = snapshot.lastUpdateId;

        if (first_increment_id < snapshot_id) {
            //TODO now i don't know how to make initialisation for ws_connection properly
            !panic!();
        }

        let bids1 = BTreeMap::from_iter(snapshot.bids);
        let asks1 = BTreeMap::from_iter(snapshot.asks);
        let mut price_levels : PriceLevels  = PriceLevels{last_update_id: snapshot_id, bids: bids1, asks : asks1  };
        
        //println!({price_levels:?});


        //     let jopa = self.stream.next().await;

        //     let jopa1 = jopa.unwrap().unwrap();
        //     stdout().write(&jopa1.into_data());
        //     stdout().flush();
        // }
    

        // let body = get_request::get_start_snapshot()?;
        // println!("body = {body:?}");
        // json_parser::json_parse::parse(&body);
    });
}