use actix_web::{get, web, HttpRequest, HttpResponse};

#[get("/user/{id}")]
async fn path_segment_param(req: HttpRequest, path: web::Path<(u32,)>) -> HttpResponse {
    // uri: /user/123, path: None, skip: 9, segments: [("id", Segment(6, 9))]
    dbg!(req.clone());
    // 注意暂时不要用`path.0.0`这样的表达式，可读性差
    let user_id: u32 = path.into_inner().0;
    println!("path = {}", user_id);
    println!("&req.match_info()[\"id\"] = {}", &req.match_info()["id"]);
    println!(
        "req.match_info().query(\"id\") = {}",
        req.match_info().query("id")
    );
    println!(
        "req.match_info().get(\"id\").unwrap() = {}",
        req.match_info().get("id").unwrap()
    );
    HttpResponse::Ok().body("ok")
}

#[cfg(not)]
async fn test_path_segment_param() {
    let mut app_service = test::init_service(App::new().service(path_segment_param)).await;
    let req = test::TestRequest::get().uri("/user/123").to_request();
    test::call_service(&mut app_service, req).await;
}

#[cfg(not)]
#[actix_web::test]
async fn main() {
    tokio_uring::start(async {
        test_path_segment_param().await;
    });
}
