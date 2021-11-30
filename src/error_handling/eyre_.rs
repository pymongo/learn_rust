use std::os::unix::prelude::RawFd;

#[test]
fn main() -> eyre::Result<()> {
    // eyre 通过 anyhow 实现所以也有 backtrace 功能
    // color_eyre 里面红色是自己 crate 的代码，绿色是其他库
    color_eyre::install()?;
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    // std::env::set_var("RUST_BACKTRACE", "1");
    dbg!(std::env::var("RUST_BACKTRACE").unwrap());
    tracing::info!("before foo");
    foo()?;
    Ok(())
}

fn foo() -> eyre::Result<()> {
    let _fd = bar()?;
    Ok(())
}

fn bar() -> eyre::Result<RawFd> {
    match unsafe { libc::open("/tmp/no_exist\0".as_ptr().cast(), libc::O_RDONLY) } {
        -1 => Err(std::io::Error::last_os_error().into()),
        fd => Ok(fd),
    }
}
