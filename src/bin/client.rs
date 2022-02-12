use {
    hyper::{header::HeaderValue, Body, Client, Method, Request, Uri},
    learn_axum::PORT,
    serde_json::json,
    std::str::FromStr,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    for _ in 0..2 {
        let json = json!({ "id": 1usize });
        let request = Request::builder()
            .method(Method::POST)
            .uri(Uri::from_str(&format!("http://{}/query_json", PORT))?)
            .header("Content-Type", HeaderValue::from_static("application/json"))
            .body(Body::from(json.to_string()))
            .unwrap();
        let res = Client::new().request(request).await?;
        let status = res.status();
        let buf = hyper::body::to_bytes(res).await?;
        println!("got status: {}, body: {buf:?}", status);
    }
    Ok(())
}
