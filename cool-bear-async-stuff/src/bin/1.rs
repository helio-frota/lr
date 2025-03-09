use std::{thread::sleep, time::Duration};

// all sync and calling sleep syscall
fn main() {
    println!("hello world");
    sleep(Duration::from_millis(500));
    println!("ok.")
}
