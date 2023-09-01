//this example is more advanced than analyze.rs. 
// Bu örnek, analyze.rs'den daha gelişmiştir.

use rustysozluk::{fetch_user, tokio};
use rustysozluk::analyzer::analyzer::{read_stopwords, word_frequencies};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = "morgomir";
    let entry_number = 4;
    let entries = fetch_user(username, entry_number).await?;
    
    let stopwords = read_stopwords("files/stopwords.csv")?;  // stopwords file path

    // Count word frequencies in the entries // kelime frekansını çıkart
    let word_count: std::collections::HashMap<String, usize> = word_frequencies(entries.iter().map(|e| e.content.clone()).collect(), &stopwords);
    
    // Sort the HashMap by value (frequency) in descending order // HashMapi azalana göre  sırala
    let mut word_vec: Vec<(&String, &usize)> = word_count.iter().collect();
    word_vec.sort_by(|a, b| b.1.cmp(a.1));

    // Display the top 10 most frequent words / en çok kullanılan 10 kelimeyi göster
    println!("Top 10 most frequent words:");
    for (idx, (word, freq)) in word_vec.iter().enumerate().take(10) {
        println!("{}. {}: {}", idx + 1, word, freq);
    }
    
    Ok(())
}