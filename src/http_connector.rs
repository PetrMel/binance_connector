pub mod get_request {
    use eyre::Error;


pub async fn get_start_snapshot_from(url : &str) -> Result<String, Error> {
    let body = reqwest::get(url)
    .await?
    .text()
    .await?;

    Ok(body)
}

}