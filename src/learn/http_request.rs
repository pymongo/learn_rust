extern crate reqwest;
use std::collections::HashMap;

#[allow(dead_code)]
const API_URL_1 : &str = "https://httpbin.org/ip";
#[allow(dead_code)]
const API_URL_2 : &str = "https://jsonplaceholder.typicode.com/posts/1";
// API_URL_2以我所学解析不了，json有的是int有的是string类型

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