use async_task::{Runnable, Task};
use futures_lite::future;
use once_cell::sync::Lazy;
use std::pin::Pin;
use std::task::Poll;
use std::time::{Duration, Instant};
use std::{panic::catch_unwind, thread};
fn main() {
    let handle_one = spawn_task(sleeping(1));
    let handle_two = spawn_task(sleeping(2));
    let handle_three = spawn_task(sleeping(3));

    println!("before the sleep");
    std::thread::sleep(Duration::from_secs(5));
    println!("before the block");

    future::block_on(handle_one);
    future::block_on(handle_two);
    future::block_on(handle_three);
}
async fn sleeping(label: u8) {
    println!("sleeping {}", label);
    AsyncSleep::new(Duration::from_secs(3)).await;
    println!("Processing sleep {}", label);
    AsyncSleep::new(Duration::from_secs(3)).await;
    println!("doing sleep {}", label);
}
fn spawn_task<F, T>(future: F) -> Task<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    static QUEUE: Lazy<flume::Sender<Runnable>> = Lazy::new(|| {
        let (tx, rx) = flume::unbounded::<Runnable>();
        thread::spawn(move || {
            while let Ok(runnable) = rx.recv() {
                let _ = catch_unwind(|| runnable.run());
            }
        });
        tx
    });
    let schedule = |runnabel| QUEUE.send(runnabel).unwrap();
    let (runnable, task) = async_task::spawn(future, schedule);
    runnable.schedule();
    task
}
struct AsyncSleep {
    start_itme: Instant,
    duration: Duration,
}

impl AsyncSleep {
    fn new(duration: Duration) -> Self {
        Self {
            start_itme: Instant::now(),
            duration,
        }
    }
}
impl Future for AsyncSleep {
    type Output = ();
    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let elapsed_time = self.start_itme.elapsed();
        if elapsed_time >= self.duration {
            Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
