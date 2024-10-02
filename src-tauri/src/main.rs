use csv::ReaderBuilder;
use serde::{Serialize, Deserialize};  // <-- Import both Serialize and Deserialize
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[derive(Serialize, Deserialize)]  // <-- Add Deserialize if you plan to deserialize
struct Record {
    columns: HashMap<String, Value>,
}

fn csv_to_json_in_time(file_path: &str, output_path: &str, start_time: i64, end_time: i64) -> Result<(), Box<dyn Error>> {
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
        let mut found_record = false; // Flag to track if we've found a matching record

        // Iterate over each header and map the corresponding value
        for (i, header) in headers.iter().enumerate() {
            let value = &record[i];

            // Check if the current header is the time column
            if header == "time" {
                // Parse the time value as an integer
                let csv_time: i64 = value.parse()?;

                // Skip rows where the time doesn't match the filter
                if csv_time < start_time || csv_time > end_time {
                    continue;
                } else {
                    found_record = true; // Mark as found if the time matches the range
                }
            }

            // Parse the value as an integer, float, or string
            let parsed_value = if let Ok(int_value) = value.parse::<i64>() {
                Value::from(int_value) // Store as integer
            } else if let Ok(float_value) = value.parse::<f64>() {
                Value::from(float_value) // Store as float
            } else {
                Value::from(value.to_string()) // Store as string if not a number
            };

            row_data.insert(header.to_string(), parsed_value);
        }

        // Add the filtered record to the result set if row_data is not empty and we've found a matching record
        if found_record && !row_data.is_empty() {
            records.push(Record { columns: row_data });
        }
    }

    // Convert records to JSON
    let json_data = serde_json::to_string_pretty(&records)?;

    // Write JSON data to the output file
    let mut file = File::create(output_path)?;
    file.write_all(json_data.as_bytes())?;

    println!(
        "Filtered CSV data successfully converted to JSON and saved to {}",
        output_path
    );
    Ok(())
}
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
