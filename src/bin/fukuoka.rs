use {hyper::Client, hyper_tls::HttpsConnector, regex::Regex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    {
        let base =
            "https://ckan.open-governmentdata.org/dataset/401000_pref_fukuoka_covid19_patients";
        let target = Regex::new("https://ckan[^\"]+csv").expect("wrong regex");
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let res = client.get(base.parse()?).await?;
        let buf: axum::body::Bytes = hyper::body::to_bytes(res).await?;
        let str = String::from_utf8_lossy(buf.as_ref());
        for l in str.lines() {
            if let Some(rurl) = target.captures(l) {
                let url = &rurl[0];
                let res = client.get(url.parse()?).await?;
                let buf = hyper::body::to_bytes(res).await?;
                let str = String::from_utf8_lossy(buf.as_ref());
                print!("{str}");
            }
        }
    }
    Ok(())
}
