use rustysozluk::{fetch_user, tokio};
use rustysozluk::analyzer::analyzer::top_words;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = "morgomir";
    let entry_number = 4;
    let output_csv = false;
    let entries = fetch_user(username, entry_number).await?;
    top_words(entries, "files/stopwords.csv", 5,output_csv)?;  // stopwords file path, top N words. False means output is not csv.
    Ok(())
}



