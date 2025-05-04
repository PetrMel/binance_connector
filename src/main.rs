use eyre::Error;

mod http_connector;
mod json_parser;
mod ws_connector;

pub use crate::http_connector::get_request;
pub use crate::json_parser::json_parse;
pub use crate::ws_connector::ws_connect_name;

//#[tokio::main]
fn main() -> Result<(), Error> {
    //let our_snapshot : json_parser::json_parse::PriceLevelsSnapshot;
    ws_connect_name::ws_connect();
    // let body = get_request::get_start_snapshot()?;
    // println!("body = {body:?}");
    // json_parser::json_parse::parse(&body);
    Ok(())
}