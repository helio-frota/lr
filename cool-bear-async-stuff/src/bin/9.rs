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

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("MyFuture::poll()");
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}
