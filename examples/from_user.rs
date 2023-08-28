// This example shows how to fetch entries from a user's page. Notice you only provide the simple username, not the full URL. 
// Bu örnekte, bir kullanıcının sayfasından girdileri nasıl alacağınız gösterilmiştir. Sadece kullanıcı adını girmeniz yeterli.

use rustysozluk::fetch_user;
use rustysozluk::tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username= "morgomir";
    let entry_number = 4; // number of entries to fetch //alınacak girdi sayısı
    let entries = fetch_user(username, entry_number).await?;
    println!("Extracted {} entries:", entries.len());
    for entry in entries.iter() {
        println!("Content: {}\nDate: {}\nUsername: {}", entry.content, entry.date, entry.username);
    }
    Ok(())
}