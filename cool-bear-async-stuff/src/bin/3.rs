use std::{thread::sleep, time::Duration};

// spawning two async tasks which contains code that blocks
// the executor.
#[tokio::main]
async fn main() {
    let one = tokio::spawn(ola());
    let two = tokio::spawn(ola());
    let (_, _) = tokio::join!(one, two);
}

async fn ola() {
    println!("hello world");
    sleep(Duration::from_millis(500));
    println!("ok.");
}
