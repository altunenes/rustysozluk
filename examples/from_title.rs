//this example shows how to extract entries from a title. Notice you have to provide the full URL.
//Bu örnekte, bir başlıktan girdileri nasıl alacağınız gösterilmiştir. Tam URL'yi girmeniz gerekmektedir.

use rustysozluk::fetch_title;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let entries = fetch_title("https://eksisozluk1923.com/rust-programlama-dili--5575227", 4).await?;
    println!("Extracted {} entries:", entries.len());
    for entry in entries.iter() {
        println!("{}", entry);
    }
    Ok(())
}