//! 抄写async-book的executor的实现
use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },
    std::{
        future::Future,
        pin::Pin,
        sync::mpsc::{sync_channel, Receiver, SyncSender},
        sync::{Arc, Mutex},
        task::{Context, Poll, Waker},
    },
};

struct TimerFuture {
    state: Arc<Mutex<TimerFutureState>>,
}

struct TimerFutureState {
    /// is completely: whether or not the sleep time has elapsed
    is_ready: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();
        if state.is_ready {
            Poll::Ready(())
        } else {
            // 保存当前Future状态机执行到哪一步的状态，赋值给waker字段，为了让代码变得简单就暂时先用clone
            // When a future is not ready yet, poll returns Poll::Pending
            // and stores a clone of the Waker copied from the current Context
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    fn new() -> Self {
        let state = Arc::new(Mutex::new(TimerFutureState {
            is_ready: false,
            waker: None,
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
        Self { state }
    }
}

/*
executor will work by sending tasks to run over a channel
executor will pull events off of the channel and run then

while a task is woken, it can schedule itself to be polled again by putting itself back onto the channel

*/

// A future can be reschedule itself to be polled by an Executor
struct Task {
    /**
    Rust isn't smart enough to know all future run in one executor thread, so we need Mutex to proof mutable thread safely
    `BoxFuture<T>` is a type alias for `Pin<Box<dyn Future<Output = T> + Send + 'static>>`.
    */
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    /// Handle to place the task itself back onto the task queue
    task_sender: SyncSender<Arc<Task>>,
}

/// waker S are responsible for scheduling a task to be polled again once wake() is called
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).unwrap()
    }
}

/// spawns new futures onto the task channel
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).unwrap();
    }
}

/// task executor that receives takes off of a channel and runs them
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                if future.as_mut().poll(context).is_pending() {
                    *future_slot = Some(future);
                }
            }
        }
    }
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUE_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUE_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

fn main() {
    let (executor, spawner) = new_executor_and_spawner();
    spawner.spawn(async {
        TimerFuture::new().await;
    });

    // drop spawner so that executor knows it is finished and won't receive more incoming tasks to run
    drop(spawner);

    executor.run();
}
