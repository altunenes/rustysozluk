mod http_client;
mod parser;
pub use http_client::fetch_page;
pub use parser::{fetch_user, fetch_title};
