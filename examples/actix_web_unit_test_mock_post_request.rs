use actix_web::{post, test, web, App, Responder};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct User {
    age: u16,
}

#[post("/create_user")] // 用get请求执行修改数据库的操作不合规范，此函数仅用于Demo/Example中演示
async fn post_form(form: web::Form<User>) -> impl Responder {
    web::Json(serde_json::json!(User { age: form.age }))
}

async fn test_post_form() {
    let mut app_service = test::init_service(App::new().service(post_form)).await;
    let user = User { age: 18 };
    let req = test::TestRequest::post()
        .uri("/create_user")
        .set_form(&user)
        .to_request();
    let resp: User = test::read_response_json(&mut app_service, req).await;
    println!("response = {:#?}", resp);
}

#[actix_web::main]
async fn main() {
    test_post_form().await;
}
