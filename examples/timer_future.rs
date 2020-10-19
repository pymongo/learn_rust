use std::{
    future::Future,
    sync::{Arc, Mutex},
    task::{Context, Waker, Poll},
    pin::Pin,
};

struct TimerFuture {
    state: Arc<Mutex<TimerFutureState>>
}

struct TimerFutureState {
    /// is completely: whether or not the sleep time has elapsed
    is_ready: bool,
    waker: Option<Waker>
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();
        if state.is_ready {
            Poll::Ready(())
        } else {
            // 保存当前Future状态机执行到哪一步的状态，赋值给waker字段，为了让代码变得简单就暂时先用clone
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    fn new() -> Self {
        let state = Arc::new(Mutex::new(TimerFutureState {
            is_ready: false,
            waker: None
        }));
        let thread_shared_state = state.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(200));
            let mut shared_state = thread_shared_state.lock().unwrap();

            // Signal that the timer has completed and wake up last task(other future?)
            shared_state.is_ready = true;
            println!("future is complete");
            if let Some(waker) = shared_state.waker.take() {
                waker.wake();
            }
        });
        Self {
            state
        }
    }
}

fn main() {
    // will also run
    TimerFuture::new();
    futures::executor::block_on(async {
        TimerFuture::new().await;
    });
}