/// This example shows how to export entries to JSON or CSV.
/// Bu örnekte, girdileri JSON veya CSV olarak nasıl dışa aktaracağınız gösterilmiştir.
use rustysozluk::tokio;
use rustysozluk::export_to_csv; //import export_json if you want to export to json //json'a aktarmak istiyorsanız export_json'ı import edin
use rustysozluk::fetch_title;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let entries = fetch_title("https://eksisozluk1923.com/rust-programlama-dili--5575227", 4).await?;
    println!("Extracted {} entries:", entries.len());
    export_to_csv(entries, "entries.csv")?; //export_to_json(entries, "entries.csv")?; if you want to export to json
 
    Ok(())
}