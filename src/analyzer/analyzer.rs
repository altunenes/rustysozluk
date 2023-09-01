use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use crate::parser::Entry;
use csv::Writer;
use csv::Reader;

/// Reads stopwords from a CSV file and returns them as a HashSet.
/// Each line in the CSV file represents a stopword.
/// Stopwords are commonly used words that are often filtered out before
/// textual data processing.
///
/// # Arguments
///
/// * `file_path` - A string slice containing the path to the stopwords CSV file.
///
/// # Returns
///
/// A `HashSet<String>` containing the stopwords.
pub fn read_stopwords(file_path: &str) -> io::Result<HashSet<String>> {
    let mut stopwords = HashSet::new();
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let word: Vec<&str> = line.split(',').collect(); 
        stopwords.insert(word[0].to_string()); 
    }
    Ok(stopwords)
}


/// Preprocesses the given text.
/// The function lowercases the text, removes non-alphabetic characters,
/// and filters out stopwords.
///
/// # Arguments
///
/// * `text` - A string slice containing the text to preprocess.
/// * `stopwords` - A reference to a HashSet containing stopwords.
///
/// # Returns
///
/// A `String` containing the preprocessed text.
pub fn preprocess_text(text: &str, stopwords: &HashSet<String>) -> String {
    text.to_lowercase()
        .chars()
        .filter(|&c| c.is_alphabetic() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .filter(|&word| !stopwords.contains(word))
        .collect::<Vec<&str>>()
        .join(" ")
}

/// Counts the frequency of each word in the given vector of strings.
/// This function calculates the frequency of each word in the given text entries
/// after preprocessing and returns a HashMap.
///
/// # Arguments
///
/// * `entries` - A vector of strings representing the entries to analyze.
/// * `stopwords` - A reference to a HashSet containing stopwords.
///
/// # Returns
///
/// A `HashMap` where the keys are words and the values are their frequencies.
pub fn word_frequencies(entries: Vec<String>, stopwords: &HashSet<String>) -> HashMap<String, usize> {
    let mut word_count: HashMap<String, usize> = HashMap::new();

    for entry in entries {
        let processed_entry = preprocess_text(&entry, stopwords);
        for word in processed_entry.split_whitespace() {
            *word_count.entry(word.to_string()).or_insert(0) += 1;
        }
    }

    word_count
}



/// Analyzes the given entries and displays the top N most frequent words.
/// The function displays the top N most frequent words in the given text entries
/// and has an option to output the result as a CSV file.
///
/// # Arguments
///
/// * `entries` - A vector of `Entry` structs representing the entries to analyze.
/// * `stopwords_file` - The path to the stopwords file.
/// * `n` - The number of top frequent words to display.
/// * `output_csv` - A boolean value that determines whether the output should be in CSV format.
///
/// # Returns
///
/// A `Result` which is either `Ok(())` if successful, or an `io::Error`.
pub fn top_words(entries: Vec<Entry>, stopwords_file: &str, n: usize, output_csv: bool) -> io::Result<()> {
    let entry_contents: Vec<String> = entries.into_iter().map(|e| e.content).collect();
    let stopwords = read_stopwords(stopwords_file)?;
    let word_count = word_frequencies(entry_contents, &stopwords);
    let mut word_vec: Vec<(&String, &usize)> = word_count.iter().collect();
    word_vec.sort_by(|a, b| b.1.cmp(a.1));

    if output_csv {
        let mut wtr = Writer::from_path("top_words.csv")?;
        wtr.write_record(&["Rank", "Word", "Frequency"])?;
        for (idx, (word, freq)) in word_vec.iter().enumerate().take(n) {
            wtr.write_record(&[(idx + 1).to_string(), word.to_string(), freq.to_string()])?;
        }
        wtr.flush()?;
        println!("CSV file generated: top_words.csv");
    } else {
        println!("Top {} most frequent words:", n);
        println!("{:<10} {:<15} {}", "Rank", "Word", "Frequency");
        println!("{}", "-".repeat(30));
        for (idx, (word, freq)) in word_vec.iter().enumerate().take(n) {
            println!("{:<10} {:<15} {}", idx + 1, word, freq);
        }
    }
    Ok(())
}


#[derive(Debug)]
 pub struct Sentiment {
    tone: f32,
    polarity: i32,
}


/// Reads a sentiment lexicon from a CSV file and returns a HashMap.
/// The function reads a CSV file and returns a HashMap where the keys are words
/// and the values are Sentiment structs containing tone and polarity.
///
/// # Arguments
///
/// * `file_path` - A string slice containing the path to the sentiment lexicon CSV file.
///
/// # Returns
///
/// A `HashMap<String, Sentiment>` containing the sentiment lexicon.

pub fn read_sentiment_lexicon(file_path: &str) -> io::Result<HashMap<String, Sentiment>> {
    let mut lexicon = HashMap::new();
    let mut rdr = Reader::from_path(file_path)?;
    for result in rdr.records() {
        let record = result?;
        let word = &record[1];
        let tone_str = &record[2].replace(",", "."); 
        let tone: f32 = tone_str.parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let polarity: i32 = record[3].parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        lexicon.insert(word.to_string(), Sentiment { tone, polarity });
    }
    Ok(lexicon)
}


/// Performs sentiment analysis on the given text entries.
/// The function performs sentiment analysis on the given text entries and prints
/// the results in a formatted manner.
///
/// # Arguments
///
/// * `entries` - A vector of `Entry` structs representing the entries to analyze.
///
/// # Returns
///
/// A `Result` which is either `Ok(())` if successful, or an `io::Error`.

pub fn analyze_sentiment(entries: Vec<Entry>) -> io::Result<()> {
    let entry_contents: Vec<String> = entries.into_iter().map(|e| e.content).collect();
    
    let stopwords_file = "files/stopwords.csv";
    let lexicon_file = "files/SWNetTR.csv";
    
    let stopwords = read_stopwords(stopwords_file)?;
    let lexicon = read_sentiment_lexicon(lexicon_file)?;
    
    let mut total_tone: f32 = 0.0;
    let mut total_positive_tone: f32 = 0.0;
    let mut total_negative_tone: f32 = 0.0;
    let mut total_polarity: i32 = 0;
    let mut total_positive_polarity: i32 = 0;
    let mut total_negative_polarity: i32 = 0;
    
    for entry in &entry_contents {
        let processed_entry = preprocess_text(entry, &stopwords);
        for word in processed_entry.split_whitespace() {
            if let Some(sentiment) = lexicon.get(word) {
                total_tone += sentiment.tone;
                total_polarity += sentiment.polarity;
                if sentiment.polarity == 1 {
                    total_positive_tone += sentiment.tone;
                    total_positive_polarity += 1;
                } else if sentiment.polarity == -1 {
                    total_negative_tone += sentiment.tone;
                    total_negative_polarity += 1;
                }
            }
        }
    }
    
    println!("+---------------------------------------+");
    println!("|            Sentiment Analysis         |");
    println!("+---------------------------------------+");
    println!("| Total Tone:                {:>10.2}  |", total_tone);
    println!("| Total Polarity:            {:>10}  |", total_polarity);
    println!("+---------------------------------------+");
    println!("| Positive Tone:             {:>10.2}  |", total_positive_tone);
    println!("| Positive Polarity Count:   {:>10}  |", total_positive_polarity);
    println!("+---------------------------------------+");
    println!("| Negative Tone:             {:>10.2}  |", total_negative_tone);
    println!("| Negative Polarity Count:   {:>10}  |", total_negative_polarity);
    println!("+---------------------------------------+");
    
    Ok(())
}