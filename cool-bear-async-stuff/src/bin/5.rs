use std::time::Duration;

use tokio::time::sleep;

// same as 4 but now doesn't block the executor
// the previous `sleep` calls the sleep syscall
// the sleep in tokio returns a Future that registers a timer
// when we first poll it and it only completes when the deadline
// is reached.
// Future is a trait and anything can be a Future.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let one = tokio::spawn(ola());
    let two = tokio::spawn(ola());
    let (_, _) = tokio::join!(one, two);
}

async fn ola() {
    println!("hello world");
    sleep(Duration::from_millis(500)).await;
    println!("ok.");
}
