use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};

struct Delay {
    when: Instant,
}

// Implement the `Future` trait for the `Delay` struct.
// This allows us to use `Delay` as a future that can be awaited.
impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        println!("Polling the future.");
        if Instant::now() >= self.when {
            println!("Future Completed.");
            Poll::Ready(())
        } else {
            let waker = cx.waker().clone();
            let when = self.when;

            std::thread::spawn(move || {
                let now = Instant::now();
                if now < when {
                    std::thread::sleep(when - now);
                }
                waker.wake();
            });

            println!("Future not ready.");
            Poll::Pending
        }
    }
}

pub async fn test() {
    println!("--- Start module: {}", module_path!());

    let delay = Delay {
        when: Instant::now() + Duration::from_secs(2),
    };

    println!("Block until the future is ready.");
    delay.await;
    println!("Future is ready, continue to execute.");

    println!("--- End module: {}", module_path!());
}
