use reqwest::Client;
use std::future::Future;
use std::time::Duration;

fn before_do() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}

fn fetch_thing(client: &'static Client, url: &'static str) -> impl Future<Output = ()> + 'static {
    async move {
        // % means Display
        tracing::info!(%url, "before req url");
        let res = client
            .get(url)
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap();
        // ? means Debug
        tracing::info!(CONTENT_LENGTH = ?res.headers().get(reqwest::header::CONTENT_LENGTH), "Got a response!");
    }
}

const URL_1: &str = "https://fasterthanli.me/articles/whats-in-the-box";
const URL_2: &str = "https://fasterthanli.me/series/advent-of-code-2020/part-13";

/// concurrency(并发) two req
// #[tokio::test]
/// 但发现用了
#[doc = "但发现用了 multi thread 之后 reqwest 每个线程都会启一个 client 导致两次 connect 系统调用"]
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    before_do();
    let client = Client::new();
    let leaked_client = Box::leak(Box::new(client));

    let fut1 = fetch_thing(leaked_client, URL_1);
    let fut2 = fetch_thing(leaked_client, URL_2);

    let handle1 = tokio::spawn(fut1);
    let handle2 = tokio::spawn(fut2);

    handle1.await.unwrap();
    handle2.await.unwrap();
}

/*
$ strace -f -e connect ./target/debug/two_req

strace: Process 60661 attached
strace: Process 60662 attached
Nov 30 15:18:35.343  INFO two_req: before req url url=https://fasterthanli.me/articles/whats-in-the-box
Nov 30 15:18:35.343  INFO two_req: before req url url=https://fasterthanli.me/series/advent-of-code-2020/part-13
strace: Process 60663 attached
strace: Process 60664 attached
[pid 60663] connect(6, {sa_family=AF_UNIX, sun_path="/var/run/nscd/socket"}, 110) = -1 ENOENT (No such file or directory)
[pid 60663] connect(6, {sa_family=AF_UNIX, sun_path="/var/run/nscd/socket"}, 110 <unfinished ...>
[pid 60664] connect(7, {sa_family=AF_UNIX, sun_path="/var/run/nscd/socket"}, 110 <unfinished ...>
[pid 60663] <... connect resumed>)      = -1 ENOENT (No such file or directory)
[pid 60664] <... connect resumed>)      = -1 ENOENT (No such file or directory)
[pid 60664] connect(6, {sa_family=AF_UNIX, sun_path="/run/systemd/resolve/io.systemd.Resolve"}, 42 <unfinished ...>
[pid 60663] connect(7, {sa_family=AF_UNIX, sun_path="/run/systemd/resolve/io.systemd.Resolve"}, 42 <unfinished ...>
[pid 60664] <... connect resumed>)      = -1 ENOENT (No such file or directory)
[pid 60663] <... connect resumed>)      = -1 ENOENT (No such file or directory)
[pid 60664] connect(6, {sa_family=AF_INET, sin_port=htons(53), sin_addr=inet_addr("192.168.18.1")}, 16) = 0
[pid 60663] connect(7, {sa_family=AF_INET, sin_port=htons(53), sin_addr=inet_addr("192.168.18.1")}, 16) = 0
[pid 60664] connect(6, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("104.21.92.169")}, 16) = 0
[pid 60664] connect(6, {sa_family=AF_UNSPEC, sa_data="\0\0\0\0\0\0\0\0\0\0\0\0\0\0"}, 16) = 0
[pid 60664] connect(6, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("172.67.196.144")}, 16) = 0
[pid 60664] connect(6, {sa_family=AF_INET6, sin6_port=htons(0), sin6_flowinfo=htonl(0), inet_pton(AF_INET6, "2606:4700:3034::6815:5ca9", &sin6_addr), sin6_scope_id=0}, 28) = -1 ENETUNREACH (Network is unreachable)
[pid 60664] connect(6, {sa_family=AF_UNSPEC, sa_data="\0\0\0\0\0\0\0\0\0\0\0\0\0\0"}, 16) = 0
[pid 60664] connect(6, {sa_family=AF_INET6, sin6_port=htons(0), sin6_flowinfo=htonl(0), inet_pton(AF_INET6, "2606:4700:3031::ac43:c490", &sin6_addr), sin6_scope_id=0}, 28) = -1 ENETUNREACH (Network is unreachable)
[pid 60662] connect(6, {sa_family=AF_INET, sin_port=htons(443), sin_addr=inet_addr("104.21.92.169")}, 16) = -1 EINPROGRESS (Operation now in progress)
[pid 60662] connect(8, {sa_family=AF_INET6, sin6_port=htons(443), sin6_flowinfo=htonl(0), inet_pton(AF_INET6, "2606:4700:3034::6815:5ca9", &sin6_addr), sin6_scope_id=0}, 28) = -1 ENETUNREACH (Network is unreachable)
[pid 60662] connect(8, {sa_family=AF_INET6, sin6_port=htons(443), sin6_flowinfo=htonl(0), inet_pton(AF_INET6, "2606:4700:3031::ac43:c490", &sin6_addr), sin6_scope_id=0}, 28) = -1 ENETUNREACH (Network is unreachable)
[pid 60663] connect(7, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("104.21.92.169")}, 16) = 0
[pid 60663] connect(7, {sa_family=AF_UNSPEC, sa_data="\0\0\0\0\0\0\0\0\0\0\0\0\0\0"}, 16) = 0
[pid 60663] connect(7, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("172.67.196.144")}, 16) = 0
[pid 60663] connect(7, {sa_family=AF_INET6, sin6_port=htons(0), sin6_flowinfo=htonl(0), inet_pton(AF_INET6, "2606:4700:3031::ac43:c490", &sin6_addr), sin6_scope_id=0}, 28) = -1 ENETUNREACH (Network is unreachable)
[pid 60663] connect(7, {sa_family=AF_UNSPEC, sa_data="\0\0\0\0\0\0\0\0\0\0\0\0\0\0"}, 16) = 0
[pid 60663] connect(7, {sa_family=AF_INET6, sin6_port=htons(0), sin6_flowinfo=htonl(0), inet_pton(AF_INET6, "2606:4700:3034::6815:5ca9", &sin6_addr), sin6_scope_id=0}, 28) = -1 ENETUNREACH (Network is unreachable)
[pid 60650] connect(7, {sa_family=AF_INET, sin_port=htons(443), sin_addr=inet_addr("104.21.92.169")}, 16) = -1 EINPROGRESS (Operation now in progress)
[pid 60650] connect(8, {sa_family=AF_INET6, sin6_port=htons(443), sin6_flowinfo=htonl(0), inet_pton(AF_INET6, "2606:4700:3031::ac43:c490", &sin6_addr), sin6_scope_id=0}, 28) = -1 ENETUNREACH (Network is unreachable)
[pid 60650] connect(8, {sa_family=AF_INET6, sin6_port=htons(443), sin6_flowinfo=htonl(0), inet_pton(AF_INET6, "2606:4700:3034::6815:5ca9", &sin6_addr), sin6_scope_id=0}, 28) = -1 ENETUNREACH (Network is unreachable)
[pid 60658] +++ exited with 0 +++
*/
