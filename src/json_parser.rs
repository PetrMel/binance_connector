pub mod json_helper {



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
pub struct PriceLevelsSnapshot {
    pub lastUpdateId: i64,
    pub bids: Vec<(String, String)>,
    pub asks: Vec<(String, String)>
}

//https://developers.binance.com/docs/binance-spot-api-docs/web-socket-streams#diff-depth-stream
// {
//     "e": "depthUpdate", // Event type
//     "E": 1672515782136, // Event time
//     "s": "BNBBTC",      // Symbol
//     "U": 157,           // First update ID in event
//     "u": 160,           // Final update ID in event
//     "b": [              // Bids to be updated
//       [
//         "0.0024",       // Price level to be updated
//         "10"            // Quantity
//       ]
//     ],
//     "a": [              // Asks to be updated
//       [
//         "0.0026",       // Price level to be updated
//         "100"           // Quantity
//       ]
//     ]
//   }
#[derive(serde::Deserialize)]
pub struct PriceLevelsIncremental {
    e: String,
    E: i64,
    s: String,
    pub U: i64,
    pub u: i64,
    pub b: Vec<(String, String)>,
    pub a: Vec<(String, String)>
}

pub fn parse_incremental(data: &str) -> Result<PriceLevelsIncremental> {
   let parsed: PriceLevelsIncremental = serde_json::from_str(data).unwrap();
    Ok(parsed)
}

pub fn parse_snapshot(data: &str) -> Result<PriceLevelsSnapshot> {
    let parsed: PriceLevelsSnapshot = serde_json::from_str(data).unwrap();
     Ok(parsed)
 }


}