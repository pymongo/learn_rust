use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const API_URL_1: &str = "https://httpbin.org/ip";
const API_URL_2: &str = "https://jsonplaceholder.typicode.com/posts/88";

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: i32,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: i32,
}

// reqwest有个feature可以设置成blocking不需要await的HTTP请求，或者用isahc库实现同步的HTTP请求
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(API_URL_1)
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);

    let resp: Post = reqwest::get(API_URL_2).await?.json().await?;
    println!("{:#?}", resp);

    Ok(())
}

#[cfg(FALSE)]
pub fn deserialize_json_str() {
    let res = serde_json::from_str(
        r#"
    {
        "userId": 1,
        "id": 1,
        "title": "reprehenderit",
        "body": "quia et susc equunturrepre"
    }
    "#,
    );
    if res.is_ok() {
        let json_value: serde_json::Value = res.unwrap();
        // let json_value: Post = res.unwrap();
        // 如果找不到json的key，会返回null
        println!("json_value[\"userId\"] = {}", json_value["userId"])
    }
}
