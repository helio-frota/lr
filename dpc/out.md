# Duplicrabs

> [!WARNING]
> Almost the same

### 🦀 1

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let one = tokio::spawn(ola());
    let two = tokio::spawn(ola());
    let (_, _) = tokio::join!(one, two);
}
```

`lr/cool-bear-async-stuff/src/bin/4.rs`

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let one = tokio::spawn(ola());
    let two = tokio::spawn(ola());
    let (_, _) = tokio::join!(one, two);
}
```

`lr/cool-bear-async-stuff/src/bin/5.rs`

### 🦀 2

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let my_fut = MyFuture {};
    println!("awaiting my_fut...");
    my_fut.await;
    println!("awaiting my_fut... done");
}
```

`lr/cool-bear-async-stuff/src/bin/7.rs`

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let my_fut = MyFuture {};
    println!("awaiting my_fut...");
    my_fut.await;
    println!("awaiting my_fut... done");
}
```

`lr/cool-bear-async-stuff/src/bin/8.rs`

### 🦀 3

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let my_fut = MyFuture {};
    println!("awaiting my_fut...");
    my_fut.await;
    println!("awaiting my_fut... done");
}
```

`lr/cool-bear-async-stuff/src/bin/7.rs`

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let my_fut = MyFuture {};
    println!("awaiting my_fut...");
    my_fut.await;
    println!("awaiting my_fut... done");
}
```

`lr/cool-bear-async-stuff/src/bin/9.rs`

### 🦀 4

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let my_fut = MyFuture {};
    println!("awaiting my_fut...");
    my_fut.await;
    println!("awaiting my_fut... done");
}
```

`lr/cool-bear-async-stuff/src/bin/8.rs`

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let my_fut = MyFuture {};
    println!("awaiting my_fut...");
    my_fut.await;
    println!("awaiting my_fut... done");
}
```

`lr/cool-bear-async-stuff/src/bin/9.rs`
