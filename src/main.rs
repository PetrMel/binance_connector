//https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints
// {
//     "lastUpdateId": 1027024,
//     "bids": [
//       [
//         "4.00000000",     // PRICE
//         "431.00000000"    // QTY
//       ]
//     ],
//     "asks": [
//       [
//         "4.00000200",
//         "12.00000000"
//       ]
//     ]
//   }


// This is using the `tokio` runtime. You'll need the following dependency:
//
// `tokio = { version = "1", features = ["full"] }`
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Some simple CLI args requirements...
    let body = reqwest::get("https://data-api.binance.vision/api/v3/depth?symbol=BNBBTC&limit=3")
    .await?
    .text()
    .await?;

    println!("body = {body:?}");

    Ok(())
}