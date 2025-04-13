use async_task::{Runnable, Task};
use futures_lite::future;
use once_cell::sync::Lazy;
use std::pin::Pin;
use std::{panic::catch_unwind, thread};

fn main() {}
fn spawn_task<F, T>(future: F) -> Task<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    todo!();
}
