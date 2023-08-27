[![Rust](https://github.com/altunenes/rustysozluk/actions/workflows/rust.yml/badge.svg)](https://github.com/altunenes/rustysozluk/actions/workflows/rust.yml) [![Latest Version]][crates.io] 

[Latest Version]: https://img.shields.io/crates/v/rustysozluk.svg
[crates.io]: https://crates.io/crates/rustysozluk




# RustySozluk

## Description

`RustySozluk` is a Rust library for fetching user entries and thread entries from eksisozluk.com.
With the power of Rust and `tokio` library, it is possible to fetch entries in a thread in a very short time.

## Features

- Fetch user entries by username
- Fetch entries in a particular thread
- Asynchronous API using Rust's `async/await`

## Installation

Add `rustysozluk` to your `Cargo.toml`:

```toml
[dependencies]
rustysozluk = "0.1.0"
```

## Usage

```rust
use rustysozluk::fetch_user;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let entries = fetch_user("morgomir", 4).await?;
    println!("Extracted {} entries:", entries.len());
    for entry in entries.iter() {
        println!("{}", entry);
    }
    Ok(())
}
```



### Contributing

Any kind of contribution is welcome! Feel free to open an issue =)