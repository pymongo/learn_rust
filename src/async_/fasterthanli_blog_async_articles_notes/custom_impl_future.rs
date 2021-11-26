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
    // only print once
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

        Poll::Pending
    }
}

#[test]
fn f2() {
    // only print once
    // program stuck/hang
    tokio_single_thread_block_on(async {
        F2.await;
    });
}

struct F3;

impl Future for F3 {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!(
            "{}: poll",
            std::any::type_name::<Self>().split("::").last().unwrap()
        );

        // cx.waker().clone().wake();
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

#[test]
fn f3() {
    // print poll repeats very quickly
    // cpu busy-wait, busy-loop
    tokio_single_thread_block_on(async {
        F3.await;
    });
}

struct F4;

impl Future for F4 {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!(
            "{}: poll",
            std::any::type_name::<Self>().split("::").last().unwrap()
        );

        let waker = cx.waker().clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(1));
            waker.wake();
        });
        Poll::Pending
    }
}

#[test]
fn f4() {
    // print poll every 1s
    tokio_single_thread_block_on(async {
        F4.await;
    });
}

/// once: Pending -> Ready
struct F5 {
    is_task_done: bool
}

impl Future for F5 {
    type Output = ();

    /// NOTE: mut self
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!(
            "{}: poll",
            std::any::type_name::<Self>().split("::").last().unwrap()
        );

        match self.is_task_done {
            false => {
                let waker = cx.waker().clone();

                std::thread::spawn(move || {
                    // do something e.g. sleep
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    waker.wake();
                });

                self.is_task_done = true;
                Poll::Pending
            },
            true => {
                Poll::Ready(())
            }
        }
    }
}

#[test]
fn f5() {
    tokio_single_thread_block_on(async {
        F5 { is_task_done: false }.await;
    });
}
