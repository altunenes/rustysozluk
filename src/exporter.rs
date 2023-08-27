//! # RustySozluk Exporter
//!
//! `rustysozluk_exporter` is a module responsible for exporting scraped data into different formats.
//! Currently, it supports exporting data into CSV and JSON formats.

use csv::Writer;
use std::fs::File;
use std::io::{Error, Write};
use serde_json;
use crate::parser::Entry;



/// Exports a list of entries to a CSV file.
///
/// The CSV file will have columns for "Content", "Date", and "Username".
///
/// # Arguments
///
/// * `entries` - A `Vec<Entry>` containing the entries to be exported.
/// * `file_name` - A `&str` specifying the name of the CSV file to be created.
///
/// # Returns
///
/// A `Result` which is either:
/// * `Ok(())` - If the export was successful.
/// * `Err(Error)` - If an IO error occurred during export.
///


pub fn export_to_csv(entries: Vec<Entry>, file_name: &str) -> Result<(), Error> {
    let file = File::create(file_name)?;
    let mut wtr = Writer::from_writer(file);
    wtr.write_record(&["Content", "Date", "Username"])?;
    for entry in entries {
        wtr.write_record(&[entry.content, entry.date, entry.username])?;
    }
    wtr.flush()?;
    Ok(())
}

/// Exports a list of entries to a JSON file.
///
/// The JSON file will contain an array of entry objects, each with "content", "date", and "username" fields.
///
/// # Arguments
///
/// * `entries` - A `Vec<Entry>` containing the entries to be exported.
/// * `file_name` - A `&str` specifying the name of the JSON file to be created.
///
/// # Returns
///
/// A `Result` which is either:
/// * `Ok(())` - If the export was successful.
/// * `Err(Error)` - If an IO error occurred during export.
///  example is similar with export_to_csv. Se the "examples" folder for the full example and usage.

pub fn export_to_json(entries: Vec<Entry>, file_name: &str) -> Result<(), Error> {
    let json_string = serde_json::to_string(&entries)?;
    let mut file = File::create(file_name)?;
    file.write_all(json_string.as_bytes())?;
    Ok(())
}