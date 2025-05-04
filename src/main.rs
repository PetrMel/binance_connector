mod http_connector;
mod json_parser;
mod ws_connector;

pub use crate::http_connector::get_request;
pub use crate::json_parser::json_parse;
pub use crate::ws_connector::ws_connector_impl;



pub struct PriceLevels {
    last_update_id: i64,
    bids: std::collections::BTreeMap<String, String>,
    asks: std::collections::BTreeMap<String, String>
}

use std::io::{stdout, Write};

fn main()  {
    trpl::run(async {

        //let price_levels : PriceLevels;
    
        let ws_url = "wss://stream.binance.com:9443/ws/bnbbtc@depth@100ms";   
        let ws_connection = ws_connector_impl::Connection::make_connection_to(&ws_url).await.unwrap();

        let message = ws_connection.get_message().await;
        let data = message.into_data();
        stdout().write(&data).unwrap();
        stdout().flush().unwrap();
        
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