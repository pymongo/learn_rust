//! 生产环境要么用isahc(基于libcurl.so)，要么用reqwest，像actix_web这种发HTTPS请求还得额外依赖openssl的，或者hyper这样的基本不用
use hyper::body::Buf;

const URL: &str = "https://jsonplaceholder.typicode.com/users/1";

async fn hyper_http_request() -> Result<(), Box<dyn std::error::Error>> {
    let res = hyper::Client::new()
        .get(URL.replace("https", "http").parse()?)
        .await?;
    let resp_body = hyper::body::aggregate(res).await?;
    let resp_json: serde_json::Value = serde_json::from_reader(resp_body.reader())?;
    println!("{}", serde_json::to_string_pretty(&resp_json)?);
    Ok(())
}

#[cfg(not)]
async fn hyper_https_request() -> Result<(), Box<dyn std::error::Error>> {
    let https_client =
        hyper::Client::builder().build::<_, hyper::Body>(hyper_tls::HttpsConnector::new());
    let resp = https_client.get(URL.parse()?).await?;
    let resp_json: serde_json::Value =
        serde_json::from_reader(hyper::body::aggregate(resp).await?.reader())?;
    println!("{}", serde_json::to_string_pretty(&resp_json)?);
    Ok(())
}

#[tokio::main]
async fn main() {
    hyper_http_request().await.unwrap();
}
