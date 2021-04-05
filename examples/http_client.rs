//! 生产环境要么用isahc(基于libcurl.so)，要么用reqwest，像actix_web这种发HTTPS请求还得额外依赖openssl的，或者hyper这样的基本不用
//! isahc异步请求可以看IgBusiness项目的谷歌验证码部分代码，isahc同步请求可以看IgBusiness项目graphql单元测试部分
//! 同事说reqwest的配置不如isahc好用，例如reqwest就没法设置请求走的代理

const URL: &str = "https://jsonplaceholder.typicode.com/posts";

fn isahc_sync() -> Result<(), Box<dyn std::error::Error>> {
    use isahc::{config::Configurable, ReadResponseExt, RequestExt};
    let resp = isahc::Request::post(URL)
        .timeout(std::time::Duration::from_secs(8))
        .header("Content-Type", "application/json")
        .body(serde_json::to_vec(&serde_json::json!({
            "title": "foo",
            "body": "bar",
            "userId": 1
        }))?)?
        .send()?
        .text()?;
    let resp: serde_json::Value = serde_json::from_str(&resp)?;
    println!("{}", serde_json::to_string_pretty(&resp)?);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    isahc_sync()?;
    Ok(())
}
