pub mod get_request {
    use eyre::Error;


pub async fn get_start_snapshot() -> Result<String, Error> {
    let body = reqwest::get("https://data-api.binance.vision/api/v3/depth?symbol=BNBBTC&limit=1")
    .await?
    .text()
    .await?;

    Ok(body)
}

}