use std::{thread::sleep, time::Duration};

// same as 3.rs but now switching to the single-threaded executor
// now it's clear we are blocking the executor
#[tokio::main(flavor = "current_thread")]
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
