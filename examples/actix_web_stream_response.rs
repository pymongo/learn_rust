use actix_web::web::get;
use actix_web::HttpResponse;

async fn response_body() -> HttpResponse {
    let str = "bytes".to_string();
    let (tx, rx_body) = actix_utils::mpsc::channel();
    let _ = tx.send(Ok::<_, actix_web::Error>(bytes::Bytes::from(str)));

    HttpResponse::Ok().streaming(rx_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .route("/", get().to(response_body))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
