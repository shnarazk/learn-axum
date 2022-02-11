use {
    hyper::{header::HeaderValue, Body, Client, Method, Request, Uri},
    hyper_tls::HttpsConnector,
    learn_axum::PORT,
    regex::Regex,
    serde_json::json,
    std::str::FromStr,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if false {
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
    }
    {
        let res = Client::new()
            .get("http://fakestoreapi.com/products".parse()?)
            .await?;
        let status = res.status();
        let buf = hyper::body::to_bytes(res).await?;
        println!("got status: {}, body: {buf:?}", status);
    }
    {
        let target = Regex::new("https://ckan[^\"]+csv").expect("wrong regex");
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let res = client
            .get(
                "https://ckan.open-governmentdata.org/dataset/401000_pref_fukuoka_covid19_patients"
                    .parse()?,
            )
            .await?;
        let status = res.status();
        println!("got status: {}", status);
        let buf: axum::body::Bytes = hyper::body::to_bytes(res).await?;
        let str = String::from_utf8_lossy(buf.as_ref());
        for l in str.lines() {
            if let Some(rurl) = target.captures(l) {
                let url = &rurl[0];
                println!("{url}");
                let res = client.get(url.parse()?).await?;
                let buf = hyper::body::to_bytes(res).await?;
                println!("{buf:?}");
            }
        }
    }
    Ok(())
}
