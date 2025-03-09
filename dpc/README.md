# dpc


> 'dpc' is a shorter name for 'duplicrabs' (which is a made-up word).

Basic and not 100% accurate for finding duplicate code blocks in Rust projects.

clone and run

```shell
cargo run --release -- -d /home/foobar/Desktop/guac-rs
```

or

```shell
cargo run --release -- -d /home/foobar/Desktop/guac-rs > out.md
```

Increase the threshold to 1 with `-t 1` to print only exactly the same code blocks

```shell
cargo run --release -- -d /home/foobar/Desktop/guac-rs -t 1
```

Ignore scanning the code on tests directory

```shell
cargo run --release -- -d /home/foobar/Desktop/guac-rs/ -i tests

or multiple dirs

cargo run --release -- -d /home/foobar/Desktop/guac-rs/ -i tests,semantic
```

[Example](./out.md)

> [!CAUTION]
> ____ ____

#### Threshold too low makes things pretty wild

cargo run --release -- -d /home/foobar/Desktop/guac-rs/ **-t 0.5**

```
### 🦀 14075

```rust
    let _result = client
        .intrinsic()
        .ingest_vulnerability(&VulnerabilityInputSpec {
            r#type: "test-vuln".to_string(),
            vulnerability_id: "ghsa-osv-cve-44".to_string(),
        })
        .await?;
```

`guac-rs/lib/tests/vulnerability.rs`

```rust
    assert_eq!(1, result.len());
    assert_eq!("test-vuln".to_string(), result[0].r#type);
    assert_eq!(
        "ghsa-osv-cve-44".to_string(),
        result[0].vulnerability_ids[0].vulnerability_id
    );
```

`guac-rs/lib/tests/vulnerability.rs`
