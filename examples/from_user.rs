// This example shows how to fetch entries from a user's page. Notice you only provide the simple username, not the full URL. 
// Bu örnekte, bir kullanıcının sayfasından girdileri nasıl alacağınız gösterilmiştir. Sadece kullanıcı adını girmeniz yeterli.

use rustysozluk::fetch_user;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let entries = fetch_user("morgomir", 4).await?;
    println!("Extracted {} entries:", entries.len());
    for entry in entries.iter() {
        println!("{}", entry);
    }
    Ok(())
}