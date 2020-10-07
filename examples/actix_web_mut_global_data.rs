use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::sync::{Arc, Mutex};

// FIXME Data中出现Arc会出现`Double Arc`问题，不推荐以下写法
#[actix_web::get("/")]
async fn count_data(counter: web::Data<Arc<Mutex<Data>>>) -> HttpResponse {
    println!("before lock");
    let mut data = counter.lock().unwrap();
    println!("after lock");
    println!("before add, data={}", data.0);
    data.0 += 1;
    println!("after add, data={}", data.0);
    HttpResponse::Ok().body(format!("{}", data.0))
}

// 摘抄自官方例子
#[actix_web::get("/app_data")]
async fn count_app_data(data: web::Data<AppData>) -> HttpResponse {
    println!("before lock");
    let mut counter = data.0.lock().unwrap();
    println!("after lock");
    println!("before add, data={}", counter);
    *counter += 1;
    println!("before add, data={}", counter);
    HttpResponse::Ok().body(format!("{}", counter))
}

// 直接用Arc<Mutex<u32>>是没法修改值的
struct Data(pub u32);
struct AppData(Mutex<i32>);

// TODO 个人认为更好的解决方案是once_cell或actomic，如果是可变全局变量就用再套一层Mutex/RwLock
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // FIXME actix2.0版本app_data在HttpServer::new的外面(main函数作用域)中初始化才能正常使用(否则发10个请求counter可能才加到2)
    let app_data = web::Data::new(AppData(Mutex::new(0)));
    // FIXME actix2.0版本data在HttpServer::new的外面(main函数作用域)中初始化才能正常使用(否则发10个请求counter可能才加到2)
    let data = Arc::new(Mutex::new(Data(0)));
    HttpServer::new(move || {
        // 这里定义的变量在多线程共享的变量很可能出错
        // let app_data = web::Data::new(AppData(Mutex::new(0)));
        App::new()
            .wrap(Logger::new("%r %s time_consuming: %Dms"))
            .data(data.clone())
            .app_data(app_data.clone())
            .service(count_data)
            .service(count_app_data)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
