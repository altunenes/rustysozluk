/// This example shows how to export entries to JSON or CSV.
/// Bu örnekte, girdileri JSON veya CSV olarak nasıl dışa aktaracağınız gösterilmiştir.

use rustysozluk::export_to_json; //import export_csv if you want to export to CSV //CSV'ye aktarmak istiyorsanız export_csv'yi import edin
use rustysozluk::fetch_title;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let entries = fetch_title("https://eksisozluk1923.com/rust-programlama-dili--5575227", 4).await?;
    println!("Extracted {} entries:", entries.len());
    export_to_json(entries, "entries.json")?; //export_to_csv(entries, "entries.csv")?; if you want to export to CSV
 
    Ok(())
}