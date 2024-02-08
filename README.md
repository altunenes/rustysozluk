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
rustysozluk = "0.1.9"
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
    let title = "https://eksisozluk.com/rust-programlama-dili--5575227"; 
    let number_of_entries = 4; 
    let entries = fetch_title(title, number_of_entries).await?;
    analyze_sentiment(entries)?;
    Ok(())

}
```


### Important Notes 📝

To properly use the analyzer module, you'll need to have access to two CSV files that serve as lexicons for sentiment analysis. These files are:

- stopwords.csv - Contains a list of Turkish stop words to be filtered out during preprocessing.
- SWNetTR.csv - Contains the sentiment lexicon based on the aforementioned model.

Both files can be found in the [files](https://github.com/altunenes/rustysozluk/tree/31d181c2241ca67c6bd4a72a5ff2bc65d7f3d395/files) folder of this GitHub repository. Download it and place it in the same directory as your project.


## Request Limitation and Rate Limiting ⚠️

When using the `rustysozluk` crate, please be mindful of the number of requests you make to eksisozluk.com. Sending an excessive number of requests in a short period of time can result in your IP address being temporarily banned from accessing the site.

### Recommendations 📋

- Rate Limiting: Implement rate limiting in your code to control the frequency of your requests.
  
- Batch Requests: If possible, batch multiple queries together to minimize the number of individual requests.
  
- Caching: Store results locally to reduce the need for repeated requests to the same URLs.
  
- By adhering to these guidelines, you help to maintain a respectful use of eksisozluk.com's resources and ensure that you can continue to benefit from the features offered by the rustysozluk library without interruption.




### Contributing

Any kind of contribution is welcome! Feel free to open an issue 🙂
