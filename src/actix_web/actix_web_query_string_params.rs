use actix_web::{get, web, HttpRequest, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    age: u16,
}

/// # 注意query_string/web::Query的params 跟 match_info/web::Path的params 是两回事
///
/// ## match_info/web::Path的使用场景：
/// 见examples/actix_web_path_segment_params.rs
///
/// ## query_string/web::Query的使用场景：
/// 见本文件
#[get("/create_user")] // 用get请求执行修改数据库的操作不合规范，此函数仅用于Demo/Example中演示
async fn query_string_params(req: HttpRequest) -> impl Responder {
    dbg!(req.clone());
    let user_params: web::Query<User> = web::Query::from_query(req.query_string()).unwrap();
    // 因为这里把user_params中的String借给了user，两个变量不能共用一个堆内存String，所以String要声明为static
    web::Json(serde_json::json!(User {
        age: user_params.age
    }))
}

#[cfg(not)]
async fn test_query_string_params() {
    let mut app_service = test::init_service(App::new().service(query_string_params)).await;
    let req = test::TestRequest::get()
        .uri("/create_user?age=18")
        .to_request();
    let resp: User = test::read_response_json(&mut app_service, req).await;
    println!("response = {:#?}", resp);
}

#[cfg(not)]
#[test]
fn main() {
    tokio_uring::start(async {
        test_query_string_params().await;
    });
}
