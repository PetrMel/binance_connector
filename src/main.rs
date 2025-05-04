use eyre::Error;

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


#[tokio::main]
async fn main() -> Result<(), Error> {

    //let price_levels : PriceLevels;
 
    let ws_url = "wss://stream.binance.com:9443/ws/bnbbtc@depth@100ms";
    let ws_connection = ws_connector_impl::Connection::make_connection_to(&ws_url).unwrap();
    
    tokio::task::spawn_blocking(|| {
        ws_connection.get_message();
    });

    // let body = get_request::get_start_snapshot()?;
    // println!("body = {body:?}");
    // json_parser::json_parse::parse(&body);
 

    Ok(())
}