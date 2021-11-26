use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[cfg(test)]
fn tokio_single_thread_block_on(fut: impl Future) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .enable_io()
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
    is_task_done: bool,
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
            }
            true => Poll::Ready(()),
        }
    }
}

#[test]
fn f5() {
    tokio_single_thread_block_on(async {
        F5 {
            is_task_done: false,
        }
        .await;
    });
}

/// warp tokio::time::sleep
/// 相当于我的 F6 就是 tokio::time::sleep 的委托/转发模式
struct F6 {
    /// Sleep::reset 可以重新设置 tokio sleep 的 timer
    sleep: Pin<Box<tokio::time::Sleep>>,
}

impl Future for F6 {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!(
            "{}: poll",
            std::any::type_name::<Self>().split("::").last().unwrap()
        );
        Pin::new(&mut self.sleep).poll(cx)
        // alternative
        // self.sleep.as_mut().poll(cx)
        // FutureExt::poll_unpin(&mut self.sleep, cx)
    }
}

#[test]
fn f6() {
    tokio_single_thread_block_on(async {
        F6 {
            sleep: Box::pin(tokio::time::sleep(std::time::Duration::from_millis(1000))),
        }
        .await;
    });
}

use tokio::io::{AsyncRead, ReadBuf};

/*
201 |         Pin::new(&mut self.reader).poll_read(cx, buf)
    |         -------- ^^^^^^^^^^^^^^^^ the trait `Unpin` is not implemented for `R`
    |         |
    |         required by a bound introduced by this call
    |
    = note: consider using `Box::pin`
note: required by `Pin::<P>::new`
   --> /home/w/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/pin.rs:484:5
    |
484 |     pub const fn new(pointer: P) -> Pin<P> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider further restricting this bound
    |
195 | impl<R: AsyncRead + std::marker::Unpin> AsyncRead for F7ReadWrapper<R> {
    |                   ++++++++++++++++++++

---

## solution 1
`reader: R` -> `reader: Pin<Box<R>>`
> 通用解决方案，tokio Sleep 和 File 都能用

## solution 2
add std::pin::Unpin trait bound
我需要 Pin 之后依然能 move/拿到 self.reader 的数据
> tokio File Unpin 可以这么解决，但是 tokio Sleep **not Unpin** 不能这么处理

大部分标准库的类型都实现了 `Unpin` 例如 String, impl Unpin for String
只有 std::marker::PhantomPinned 实现了 `!Unpin`

Pin<&mut T> of it, we can never use it unpinned (ie, as &mut T) ever again, unless it implements Unpin.
所以 Sleep !Unpin 的不能通过 Pin<&mut T> 拿到 &mut T

---

像 Sleep 这让 !Unpin 的，有时候可以通过 Pin::new_unchecked 绕开编译器检查
*/
struct F7ReadWrapper<R> {
    reader: R,
}
impl<R> AsyncRead for F7ReadWrapper<R> where R: AsyncRead + std::marker::Unpin {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.reader).poll_read(cx, buf)
    }
}

/// contains both Unpin and !Unpin filed
/// map_unchecked_mut 或者 pin_project 可以拿到一个部分 !Unpin 结构体的字段，并当作 Unpin 去使用
struct F8BothUnpinAndNotUnpinField {
    sleep: tokio::time::Sleep,
    file: tokio::fs::File
}

impl AsyncRead for F8BothUnpinAndNotUnpinField
// where
//     R: AsyncRead + Unpin,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let sleep = unsafe { self.as_mut().map_unchecked_mut(|this| &mut this.sleep) };
        match sleep.poll(cx) {
            Poll::Ready(_) => {
                // 因为 sleep 字段 not unpin 传染到整个 struct 都成了 not unpin
                // let sleep = Pin::new(&mut self.sleep).poll(cx);
                let sleep = unsafe { self.as_mut().map_unchecked_mut(|this| &mut this.sleep) };

                sleep.reset(tokio::time::Instant::now() + std::time::Duration::from_millis(25));
                let reader = unsafe { self.as_mut().map_unchecked_mut(|this| &mut this.file) };
                reader.poll_read(cx, buf)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

/**
为什么 tokio Sleep 设计成 Not Unpin
因为 tokio::fs::File 相比定时器更像是无状态的，event waker 像是全局的
而 Sleep 用的 timer 是线程绑定的定时器?

因为 timer 要 register/deregisters 注册和取消注册事件
*/
#[test]
fn tokio_sleep_panic_after_move() {
    use std::{mem::swap, pin::Pin, task::Poll, time::Duration};
    use tokio::{macros::support::poll_fn, time::sleep};
    tokio_single_thread_block_on(async {
        let mut sleep1 = sleep(Duration::from_secs(1));
        let mut sleep2 = sleep(Duration::from_secs(1));

        {
            // let's use `sleep1` pinned exactly _once_
            let mut sleep1 = unsafe { Pin::new_unchecked(&mut sleep1) };

            // this creates a future whose poll method is the closure argument
            poll_fn(|cx| {
                // we poll `sleep1` once, throwing away the result...
                let _ = sleep1.as_mut().poll(cx);

                // ...and resolve immediately
                Poll::Ready(())
            })
            .await;
        }

        // then, let's use `sleep1` unpinned:
        swap(&mut sleep1, &mut sleep2);
        // by this point, `sleep1` has switched places with `sleep2`

        // finally, let's await both sleep1 and sleep2
        sleep1.await;
        sleep2.await;
    })
}
