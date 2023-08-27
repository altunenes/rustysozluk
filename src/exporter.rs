use csv::Writer;
use std::fs::File;
use std::io::{Error, Write};
use serde_json;

/// Export entries to a CSV file.
///
/// # Arguments
///
/// * `entries` - A `Vec<String>` containing the entries to be exported.
/// * `file_name` - A `&str` specifying the name of the CSV file.
///
/// # Returns
///
/// A `Result` which is either:
/// * `Ok(())` - If the export was successful.
/// * `Err(Error)` - If an IO error occurred during export.
pub fn export_to_csv(entries: Vec<String>, file_name: &str) -> Result<(), Error> {
    let file = File::create(file_name)?;
    let mut wtr = Writer::from_writer(file);
    wtr.write_record(&["Entries"])?;
    for entry in entries {
        wtr.write_record(&[entry])?;
    }
    wtr.flush()?;
    Ok(())
}
/// Export entries to a JSON file.
///
/// # Arguments
///
/// * `entries` - A `Vec<String>` containing the entries to be exported.
/// * `file_name` - A `&str` specifying the name of the JSON file.
///
/// # Returns
///
/// A `Result` which is either:
/// * `Ok(())` - If the export was successful.
/// * `Err(Error)` - If an IO error occurred during export.
pub fn export_to_json(entries: Vec<String>, file_name: &str) -> Result<(), Error> {
    let json_string = serde_json::to_string(&entries)?;
    let mut file = File::create(file_name)?;
    file.write_all(json_string.as_bytes())?;
    Ok(())
}


