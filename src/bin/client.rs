use {
    hyper::{header::HeaderValue, Body, Client, Method, Request, Uri},
    learn_axum::PORT,
    serde_json::json,
    std::str::FromStr,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let json = json!({ "id": 1 });
    dbg!(format!("{}/query_json", PORT));
    let mut request = Request::builder()
        .method(Method::POST)
        .uri(Uri::from_str(&format!("http://{}/query_json", PORT))?)
        .body(Body::from(json.to_string()))
        .unwrap();
    request
        .headers_mut()
        .insert("Content-Type", HeaderValue::from_static("application/json"));
    let res = Client::new().request(request).await?;
    // And then, if the request gets a response...
    println!("status: {}", res.status());
    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(res).await?;

    println!("body: {:?}", buf);
    Ok(())
}
