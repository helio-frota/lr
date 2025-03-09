use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let my_fut = MyFuture {};
    my_fut.await
}

struct MyFuture {}

impl Future for MyFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}
