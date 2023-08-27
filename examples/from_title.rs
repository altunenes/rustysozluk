//this example shows how to extract entries from a title. Notice you have to provide the full URL.
//Bu örnekte, bir başlıktan girdileri nasıl alacağınız gösterilmiştir. Tam URL'yi girmeniz gerekmektedir.

use rustysozluk::fetch_title;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let title = "https://eksisozluk1923.com/rust-programlama-dili--5575227"; // title URL //başlık URL'si
    let number_of_entries = 4; // number of entries to fetch //alınacak girdi sayısı
    let entries = fetch_title(title, number_of_entries).await?;
    println!("Extracted {} entries:", entries.len());
    for entry in entries.iter() {
        println!("Content: {}\nDate: {}\nUsername: {}", entry.content, entry.date, entry.username);
    }
    Ok(())
}