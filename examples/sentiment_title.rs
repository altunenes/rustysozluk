//this example fetches the title and analyzes the sentiment of the entries in the title
//bu örnek başlığı alır ve başlıktaki girdilerin duygu durumunu analiz eder
use rustysozluk::tokio;
use rustysozluk::fetch_title;
use rustysozluk::analyzer::analyzer::analyze_sentiment; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let title = "https://eksisozluk111.com/rust-programlama-dili--5575227"; // title URL //başlık URL'si
    let number_of_entries = 4; // number of entries to fetch //alınacak girdi sayısı
    let entries = fetch_title(title, number_of_entries).await?;
    analyze_sentiment(entries)?;
    Ok(())

}