use csv::ReaderBuilder;
use serde::Serialize;
use serde_json::{Value};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Debug, Serialize)]
struct Record {
    columns: HashMap<String, Value>, // Use serde_json::Value to store different data types
}
//to convert csv to json
fn csv_to_json(file_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    // Get the headers (first row of the CSV)
    let headers = rdr.headers()?.clone();

    // Collect records dynamically based on headers
    let mut records: Vec<Record> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let mut row_data = HashMap::new();

        // Map each header to its corresponding value in the row
        for (i, header) in headers.iter().enumerate() {
            let value = &record[i];

            // Try to parse the value as an integer first, then as a float
            let parsed_value = if let Ok(int_value) = value.parse::<i64>() {
                Value::from(int_value) // Store as integer
            } else if let Ok(float_value) = value.parse::<f64>() {
                Value::from(float_value) // Store as float
            } else {
                Value::from(value.to_string()) // Store as string if not a number
            };

            row_data.insert(header.to_string(), parsed_value);
        }

        records.push(Record { columns: row_data });
    }

    // Convert records to JSON
    let json_data = serde_json::to_string_pretty(&records)?;

    // Write JSON data to the output file
    let mut file = File::create(output_path)?;
    file.write_all(json_data.as_bytes())?;

    println!(
        "CSV data successfully converted to JSON and saved to {}",
        output_path
    );
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
