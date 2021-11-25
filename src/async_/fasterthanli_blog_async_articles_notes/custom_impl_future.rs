use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[cfg(test)]
fn tokio_single_thread_block_on(fut: impl Future) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    rt.block_on(fut);
}

struct F1;

impl Future for F1 {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!(
            "{}: poll",
            std::any::type_name::<Self>().split("::").last().unwrap()
        );
        Poll::Ready(())
    }
}

#[test]
fn f1() {
    tokio_single_thread_block_on(async {
        F1.await;
    });
}

struct F2;

impl Future for F2 {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!(
            "{}: poll",
            std::any::type_name::<Self>().split("::").last().unwrap()
        );
        todo!()
    }
}
