use hyper::{header::HeaderValue, Body, Client, Method, Uri, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut request = Request::builder()
        .method(Method::POST)
        .uri(Uri::from_static("http://0.0.0.0:3000/query_json"))
        .body(Body::from("{\"a\":\"3\"}"))
        .unwrap();
    request.headers_mut().insert("Content-Type", HeaderValue::from_static("application/json"));
    let res = Client::new().request(request).await?;
    // And then, if the request gets a response...
    println!("status: {}", res.status());
    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(res).await?;

    println!("body: {:?}", buf);
    Ok(())
}
