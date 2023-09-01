
//this example fetches the title and analyzes the sentiment of the entries in the title
//bu örnek başlığı alır ve başlıktaki girdilerin duygu durumunu analiz eder
use rustysozluk::{fetch_user, tokio};
use rustysozluk::analyzer::analyzer::analyze_sentiment; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = "morgomir"; // username //kullanıcı adı
    let entry_number = 4; // number of entries to fetch //alınacak girdi sayısı
    let entries = fetch_user(username, entry_number).await?;
    analyze_sentiment(entries)?; // analyze sentiment //duygu durumunu analiz et
    Ok(())
}