pub mod json_parse {


use serde::{Deserialize, Serialize};
use serde_json::Result;


//https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints
// {
//     "lastUpdateId": 1027024,
//     "bids": [
//       [
//         "4.00000000",
//         "431.00000000"
//       ]
//     ],
//     "asks": [
//       [
//         "4.00000200",
//         "12.00000000"
//       ]
//     ]
//}
#[derive(serde::Deserialize)]
struct PriceLevels {
    lastUpdateId: i64,
    bids: Vec<(String, String)>,
    asks: Vec<(String, String)>
}

pub fn parse(data: &String) -> Result<()> {
    let p: PriceLevels = serde_json::from_str(data)?;

    Ok(())
}
}