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

        if (inc_upd.U > self.last_update_id) {
            !panic!("Something went wrong");
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

        // for (bid in inc_upd.b) {
        //     if bid[1] == 0 {
        //         self.bids.remove(bid[0]);
        //         continue;
        //     }

        //     self.bids.insert(ask[0], ask[1]);
        // }

    }


}


use std::{collections::BTreeMap, io::{stdout, Write}};

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
    !panic!();

}


fn main()  {
    trpl::run(async {


        let ws_url = "wss://stream.binance.com:9443/ws/bnbbtc@depth@100ms";   
        let ws_connection = ws_connector_impl::Connection::make_connection_to(&ws_url).await.unwrap();

        let message = ws_connection.get_message().await;
      //  let data = message.into_data();
        let first_incremental = json_helper::parse_incremental(message.to_text().unwrap()).unwrap();
        let first_increment_id: i64 = first_incremental.U;

        let snapshot = get_first_snapshot(first_increment_id).await;
        let bids1 = BTreeMap::from_iter(snapshot.bids);
        let asks1 = BTreeMap::from_iter(snapshot.asks);
        let mut price_levels : PriceLevels  = PriceLevels{last_update_id: snapshot.lastUpdateId, bids: bids1, asks : asks1  };
        
        println!("{price_levels:?}");

        price_levels.update_from_incremental(first_incremental);

        println!("{price_levels:?}");

        // let fut1 = async {
            
        // };




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