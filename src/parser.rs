//! # RustySozluk Parser
//!
//! `rustysozluk_parser` is a module responsible for parsing HTML content to extract
//! relevant data such as user entries from a given web page.


use scraper::{Html, Selector,ElementRef};
use crate::http_client;


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


/// Extracts entries from the HTML content.
///
/// # Arguments
///
/// * `html` - A `str` slice that holds the HTML document.
/// * `limit` - A `usize` that defines the maximum number of entries to extract.
///
/// # Returns
///
/// A `Vec<String>` containing the extracted entries.

fn extract_entries(html: &str, limit: usize) -> Vec<String> {
    let document = Html::parse_document(html);
    let entry_selector = Selector::parse("li[data-id]").unwrap(); 
    let content_selector = Selector::parse("div.content").unwrap(); 
    let mut entries = Vec::new();
    for entry in document.select(&entry_selector).take(limit) {
        if let Some(content_element) = entry.select(&content_selector).next() {
            let content = clean_html_content(&content_element);
            entries.push(content);
        }
    }

    entries
}

/// Fetches and aggregates entries up to a given limit.
///
/// # Arguments
///
/// * `base_url` - A `&str` that defines the base URL of the page.
/// * `limit` - A `usize` that defines the maximum number of entries to fetch.
///
/// # Returns
///
/// A `Result` which is either:
/// * `Ok(Vec<String>)` - A vector containing the fetched entries.
/// * `Err(RustySozlukError)` - An error of type `RustySozlukError`.


pub async fn fetch_title(base_url: &str, limit: usize) -> Result<Vec<String>, RustySozlukError> {
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
/// A `Result` which is either:
/// * `Ok(Vec<String>)` - A vector containing the fetched entries.
/// * `Err(RustySozlukError)` - An error of type `RustySozlukError`.

pub async fn fetch_user(username: &str, limit: usize) -> Result<Vec<String>, RustySozlukError> {
    if limit == 0 {
        return Err(RustySozlukError::Other("Limit cannot be zero".to_string()));
    }
    let mut all_entries = Vec::new();
    let mut current_page = 1;
    while all_entries.len() < limit {
        let user_url = format!("https://eksisozluk1923.com/son-entryleri?nick={}&p={}", username, current_page);
        let client = reqwest::Client::new();
        let response = client.get(&user_url)
            .header("User-Agent", "your-user-agent-here")
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


#[derive(Debug, thiserror::Error)]
pub enum RustySozlukError {
    #[error("Network error occurred")]
    NetworkError,
    #[error("Other error: {0}")]
    Other(String),
}


