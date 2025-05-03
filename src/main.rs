use eyre::Error;

mod http_connector;
mod json_parser;

pub use crate::http_connector::get_request;
pub use crate::json_parser::json_parse;

//#[tokio::main]
fn main() -> Result<(), Error> {

    let body = get_request::get_start_snapshot()?;
    println!("body = {body:?}");
    json_parser::json_parse::parse(&body);
    Ok(())
}