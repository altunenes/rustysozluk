[![Rust](https://github.com/altunenes/rustysozluk/actions/workflows/rust.yml/badge.svg)](https://github.com/altunenes/rustysozluk/actions/workflows/rust.yml) [![Latest Version]][crates.io] [![Docs.rs](https://docs.rs/rustysozluk/badge.svg)](https://docs.rs/rustysozluk/latest/rustysozluk/)


[Latest Version]: https://img.shields.io/crates/v/rustysozluk.svg
[crates.io]: https://crates.io/crates/rustysozluk  



# RustySozluk

![Firefly rustysozluk-rust programming, sour, crabs with green lemons 67820](https://github.com/altunenes/rustysozluk/assets/54986652/7f70cad0-1e9c-4ed6-871d-163a485f1294)


## Description

`RustySozluk` is a Rust library for fetching user entries and thread entries from eksisozluk.com and analyzing sentiment of entries.
With the power of Rust and `tokio` library, it is possible to fetch entries in a thread in a very short time.

## Features

- Fetch user entries by username
- Fetch entries in a particular thread
- Asynchronous API using Rust's `async/await`
- Export entries to both `JSON` and `CSV` formats
- Calculate sentiment of entries or get simple frequency of words in entries
  
## Installation

Add `rustysozluk` to your `Cargo.toml`:

```toml
[dependencies]
rustysozluk = "0.1.7"
```

## Usage

```rust
use rustysozluk::{fetch_user, tokio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username= "morgomir"; // username to fetch //alınacak kullanıcı adı
    let entry_number = 4; // number of entries to fetch //alınacak girdi sayısı
    let entries = fetch_user(username, entry_number).await?;
    println!("Extracted {} entries:", entries.len());
    for entry in entries.iter() {
        println!("Content: {}\nDate: {}\nUsername: {}", entry.content, entry.date, entry.username);
    }
    Ok(())
}
```

If you want to fetch entries in a thread, you can simple use `fetch_thread` function just like `fetch_user` function, no need to change anything.



## Sentiment Analysis 

rustysozluk has "analyzer" module which is used for sentiment analysis. It uses [Sağlam et al., 2019](https://journals.tubitak.gov.tr/cgi/viewcontent.cgi?article=1639&context=elektrik) model to classify entries as positive, negative and give a "Tone" score between -1 and 1. 

here is an example usage:

```rust
use rustysozluk::tokio;
use rustysozluk::fetch_title;
use rustysozluk::analyzer::analyzer::analyze_sentiment; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let title = "https://eksisozluk1923.com/rust-programlama-dili--5575227"; 
    let number_of_entries = 4; 
    let entries = fetch_title(title, number_of_entries).await?;
    analyze_sentiment(entries)?;
    Ok(())

}
```


### Contributing

Any kind of contribution is welcome! Feel free to open an issue =)
