use bytes::buf::BufExt;

const URL: &str = "https://jsonplaceholder.typicode.com/users/1";

async fn simple_http_request() -> Result<(), Box<dyn std::error::Error>> {
    let res = hyper::Client::new()
        .get(URL.replace("https", "http").parse()?)
        .await?;
    let res_body = hyper::body::aggregate(res).await?;
    let res_json: serde_json::Value = serde_json::from_reader(res_body.reader())?;
    dbg!(res_json);
    Ok(())
}

#[cfg(FALSE)]
// #[tokio::test(core_threads = 1)]
async fn hyper_https_request() -> Result<(), Box<dyn std::error::Error>> {
    let https_client =
        hyper::Client::builder().build::<_, hyper::Body>(hyper_tls::HttpsConnector::new());
    let res = https_client.get(URL.parse()?).await?;
    let res_json: serde_json::Value =
        serde_json::from_reader(hyper::body::aggregate(res).await?.reader())?;
    dbg!(res_json);
    Ok(())
}

#[tokio::main]
async fn main() {
    simple_http_request().await.unwrap();
}
