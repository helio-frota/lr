use std::{thread::sleep, time::Duration};

// sync code with executor provided by tokio async runtime.
#[tokio::main]
async fn main() {
    println!("hello world");
    sleep(Duration::from_millis(500));
    println!("ok");
}
