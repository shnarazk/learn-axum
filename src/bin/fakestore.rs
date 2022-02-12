use {hyper::Client, hyper_tls::HttpsConnector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let res = client
        .get("http://fakestoreapi.com/products".parse()?)
        .await?;
    let buf = hyper::body::to_bytes(res).await?;
    let st = String::from_utf8_lossy(buf.as_ref());
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&st) {
        println!("{}", json);
    }
    Ok(())
}
