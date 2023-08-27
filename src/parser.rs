//! # RustySozluk Parser
//!
//! `rustysozluk_parser` is a module responsible for parsing HTML content to extract
//! relevant data such as user entries, including content, date, and username, from a given web page.



use scraper::{Html, Selector,ElementRef};
use crate::http_client;
use serde::Serialize;

/// Removes HTML tags and returns plain text content.
///
/// # Arguments
///
/// * `element` - A reference to an `ElementRef`.
///
/// # Returns
///
/// A `String` containing the cleaned-up text.

fn clean_html_content(element: &ElementRef) -> String {
    element.text().collect::<Vec<_>>().concat().trim().to_string()
}


/// Struct to hold an individual entry.
///
/// It contains the content, date, and username associated with an entry.
#[derive(Debug, Serialize)]
pub struct Entry {
    pub content: String,
    pub date: String,
    pub username: String,
}

/// Extracts entries from the HTML content.
///
/// # Arguments
///
/// * `html` - A `str` slice that holds the HTML document.
/// * `limit` - A `usize` that defines the maximum number of entries to extract.
///
/// # Returns
///
/// A `Vec<Entry>` containing the extracted entries.

fn extract_entries(html: &str, limit: usize) -> Vec<Entry> {
    let document = Html::parse_document(html);
    let entry_selector = Selector::parse("li[data-id]").unwrap();
    let content_selector = Selector::parse("div.content").unwrap();
    let date_selector = Selector::parse("a.entry-date").unwrap();
    let username_selector = Selector::parse("div#entry-author a.entry-author").unwrap(); 

    let mut entries = Vec::new();

    for entry in document.select(&entry_selector).take(limit) {
        if let Some(content_element) = entry.select(&content_selector).next() {
            if let Some(date_element) = entry.select(&date_selector).next() {
                if let Some(username_element) = entry.select(&username_selector).next() {
                    let content = clean_html_content(&content_element);
                    let date = clean_html_content(&date_element);
                    let username = clean_html_content(&username_element);
                    entries.push(Entry { content, date, username });
                }
            }
        }
    }

    entries
}

/// Fetches and aggregates entries for a given title up to a limit.
///
/// # Arguments
///
/// * `base_url` - A `&str` that defines the base URL of the page.
/// * `limit` - A `usize` that defines the maximum number of entries to fetch.
///
/// # Returns
///
/// A `Result` which is:
/// * `Ok(Vec<Entry>)` - A vector containing the fetched entries.
/// * `Err(RustySozlukError)` - An error of type `RustySozlukError`.

pub async fn fetch_title(base_url: &str, limit: usize) -> Result<Vec<Entry>, RustySozlukError> {
    if limit == 0 {
        return Err(RustySozlukError::Other("Limit cannot be zero".to_string()));
    }
    let mut all_entries = Vec::new();
    let mut current_page = 1;
    while all_entries.len() < limit {
        let page_url = format!("{}?p={}", base_url, current_page);
        let page_html = http_client::fetch_page(&page_url)
        .await
        .map_err(|_| RustySozlukError::NetworkError)?;
        let mut entries = extract_entries(&page_html, limit - all_entries.len());
        if entries.is_empty() {
            break;
        }
        all_entries.append(&mut entries);
        current_page += 1;
    }
    Ok(all_entries)
}

/// Fetches a user's entries up to a given limit.
///
/// # Arguments
///
/// * `username` - A `&str` that defines the username of the target user.
/// * `limit` - A `usize` that defines the maximum number of entries to fetch.
///
/// # Returns
///
/// A `Result` which is:
/// * `Ok(Vec<Entry>)` - A vector containing the fetched entries.
/// * `Err(RustySozlukError)` - An error of type `RustySozlukError`.

pub async fn fetch_user(username: &str, limit: usize) -> Result<Vec<Entry>, RustySozlukError> {
    if limit == 0 {
        return Err(RustySozlukError::Other("Limit cannot be zero".to_string()));
    }
    let mut all_entries = Vec::new();
    let mut current_page = 1;
    while all_entries.len() < limit {
        let user_url = format!("https://eksisozluk1923.com/son-entryleri?nick={}&p={}", username, current_page);
        let client = reqwest::Client::new();
        let response = client.get(&user_url)
            .header("X-Requested-With", "XMLHttpRequest")
            .send()
            .await
            .map_err(|_| RustySozlukError::NetworkError)?;
        let body = response.text().await
            .map_err(|_| RustySozlukError::NetworkError)?;
        let mut entries = extract_entries(&body, limit - all_entries.len());
        if entries.is_empty() {
            break;
        }
        all_entries.append(&mut entries);
        current_page += 1;
    }

    Ok(all_entries)
}

/// Custom errors for RustySozluk.
///
/// It contains errors related to network issues or other unspecified errors but this can be extended in the future.
#[derive(Debug, thiserror::Error)]
pub enum RustySozlukError {
    #[error("Network error occurred")]
    NetworkError,
    #[error("Other error: {0}")]
    Other(String),
}


