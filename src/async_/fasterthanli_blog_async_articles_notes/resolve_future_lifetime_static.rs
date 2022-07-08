use std::future::Future;
struct MyClient;

impl MyClient {
    async fn send_req(&self) {
        // 只是为了用下 self
        // dbg!(std::any::type_name_of_val(self));

        tracing::info!("before req");

        // 模拟请求的 IO 事件等待 socket response
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        tracing::info!("after  req");
    }
}

// 因为异步函数的引用入参不能保证活的比当前作用域/线程短，而且也可能会被多个线程执行
// 所以要求引用是 static 的
fn fetch(client: &'static MyClient) -> impl Future<Output = ()> + 'static {
    async move {
        client.send_req().await;
    }
}

/// 并发的发两个请求的方法: spawn 两个协程去干
/// 但要求 client 的生命周期是 static
#[cfg(FALSE)]
async fn main() {
    let client = Client;
    let fut1 = fetch(&client);
    let fut2 = fetch(&client);
    tokio::spawn(fut1);
    tokio::spawn(fut2);
    // sleep or wait two JoinHandle
}

fn before_do() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}

async fn main_transmute_extend_lifetime() {
    before_do();
    let client = MyClient;
    let client = unsafe { std::mem::transmute(&client) };
    let fut1 = fetch(client);
    let fut2 = fetch(client);
    let handle1 = tokio::spawn(fut1);
    let handle2 = tokio::spawn(fut2);

    handle1.await.unwrap();
    handle2.await.unwrap();
    // sleep or wait two JoinHandle
    // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}

#[tokio::test]
async fn test_main_transmute_extend_lifetime() {
    main_transmute_extend_lifetime().await;
}

async fn main_box_leak_lifetime_to_static() {
    before_do();
    let client = MyClient;
    let client = Box::leak(Box::new(client));
    let fut1 = fetch(client);
    let fut2 = fetch(client);
    let _handle1 = tokio::spawn(fut1);
    let _handle2 = tokio::spawn(fut2);
    // sleep or wait two JoinHandle
    // 这样就不是并发请求
    // handle1.await.unwrap();
    // handle2.await.unwrap();
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn test_main_box_leak_lifetime_to_static() {
    // let a = tokio::runtime::Handle::current().fla;
    main_box_leak_lifetime_to_static().await;
}

// 方法三: 套上 Arc
// 方法一二无论是 transmute 还是 Box::leak 似乎难以在编译期保证 Client drop 之后没有继续调用
