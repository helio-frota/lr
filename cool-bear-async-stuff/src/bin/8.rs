use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let my_fut = MyFuture {};
    println!("awaiting my_fut...");
    my_fut.await;
    println!("awaiting my_fut... done");
}

struct MyFuture {}

impl Future for MyFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}
