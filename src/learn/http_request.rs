extern crate reqwest;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
const API_URL_1 : &str = "https://httpbin.org/ip";
#[allow(dead_code)]
const API_URL_2 : &str = "https://jsonplaceholder.typicode.com/posts/88";


#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
struct Post {
  id: Option<i32>,
  title: String,
  body: String,
  #[serde(rename = "userId")]
  user_id: i32,
}


#[allow(dead_code)]
#[tokio::main]
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
  let resp = reqwest::get(API_URL_1)
    .await?
    .json::<HashMap<String, String>>()
    .await?;
  println!("{:#?}", resp);
  Ok(())
}

#[allow(dead_code)]
pub fn deserialize_json_str() {
  let json_str : &str = r#"
    {
      "userId": 1,
      "id": 1,
      "title": "reprehenderit",
      "body": "quia et susc equunturrepre"
    }
  "#;
  let res = serde_json::from_str(json_str);
  if res.is_ok() {
    let json_value : serde_json::Value = res.unwrap();
    println!("json_value[\"userId\"] = {}", json_value["userId"])
  }
}

#[allow(dead_code)]
#[tokio::main]
pub async fn json_request() -> Result<(), Box<dyn std::error::Error>> {
  let resp = reqwest::get(API_URL_2)
    .await?
    .json()
    .await?;
  println!("{:#?}", resp);
  Ok(())
}