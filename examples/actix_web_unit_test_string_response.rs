use actix_web::{get, test, App, Responder};

#[get("/")]
pub async fn get_index() -> impl Responder {
    "Hello World"
}

#[actix_web::main]
async fn main() {
    let mut app_service = test::init_service(App::new().service(get_index)).await;
    let req = test::TestRequest::default().to_request();
    let resp = test::call_service(&mut app_service, req).await;
    assert!(resp.status().is_success());
    let resp_body = test::read_body(resp).await;
    let resp_string = std::str::from_utf8(&resp_body).unwrap();
    println!("response = {}", resp_string);
    assert_eq!(resp_string, "Hello World")
}
