pub mod json_parse {



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


// #[derive(serde::Deserialize)]
// pub struct PriceLevelsSnapshot {
//     lastUpdateId: i64,
//     bids: Vec<(String, String)>,
//     asks: Vec<(String, String)>
// }

// #[derive(serde::Deserialize)]
// struct PriceLevelsIncremental {
//     e: String,
//     E: i64,
//     s: String,
//     U: String,
//     u: String,
//     b: Vec<(String, String)>,
//     a: Vec<(String, String)>
// }

pub fn parse(data: &String) -> Result<()> {
   // let p: PriceLevels = serde_json::from_str(data)?;
    println!("{data}");
    Ok(())
}
}