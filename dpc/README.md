# dpc


> 'dpc' is a shorter name for 'duplicrabs' (which is a made-up word).

Basic and not 100% accurate for finding duplicate code blocks in Rust projects.

clone and run

```shell
cargo run --release -- -d /home/foobar/Desktop/lr
```

or

```shell
cargo run --release -- -d /home/foobar/Desktop/lr > out.md
```

Increase the threshold to 1 with `-t 1` to print only exactly the same code blocks

```shell
cargo run --release -- -d /home/foobar/Desktop/lr -t 1
```

Ignore scanning the code on tests directory

```shell
cargo run --release -- -d /home/foobar/Desktop/lr/ -i tests

or multiple dirs

cargo run --release -- -d /home/foobar/Desktop/lr/ -i tests,foobar
```

[Output example](./out.md)

