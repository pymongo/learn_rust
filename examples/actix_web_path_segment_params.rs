use actix_web::{get, test, web, App, HttpRequest, HttpResponse};

#[get("/user/{id}")]
fn path_segment_param(req: HttpRequest, path: web::Path<(u32,)>) -> HttpResponse {
    // uri: /user/123, path: None, skip: 9, segments: [("id", Segment(6, 9))]
    dbg!(req.clone());
    // 注意暂时不要用: PR#77774
    /*
    注意暂时不要用`path.0.0`这样的表达式，不仅可读性差，而且会让fmt产生歧义而panic
    我看到rust的PR#77774修复了该问题，昨天被merge了
    我粗略的看了下改动，编译器的parser对表达式(INTEGER_LITERAL DOT INTEGER_LITERAL)例如`0.0`容易产生歧义:
    到底是float呢还是index of nested unnamed struct
    */
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

async fn test_path_segment_param() {
    let mut app_service = test::init_service(App::new().service(path_segment_param)).await;
    let req = test::TestRequest::get().uri("/user/123").to_request();
    test::call_service(&mut app_service, req).await;
}

#[actix_web::main]
async fn main() {
    test_path_segment_param().await;
}
