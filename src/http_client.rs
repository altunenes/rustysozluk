//! # RustySozluk HTTP Client
//!
//! `rustysozluk_http_client` is a module responsible for making HTTP requests to fetch web pages.

use reqwest::Error;

/// Fetches the content of a web page.
///
/// # Arguments
///
/// * `url` - A `&str` that defines the URL of the web page to fetch.
///
/// # Returns
///
/// A `Result` which is either:
/// * `Ok(String)` - A `String` containing the HTML content of the web page.
/// * `Err(Error)` - An error of type `reqwest::Error`.
pub async fn fetch_page(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}