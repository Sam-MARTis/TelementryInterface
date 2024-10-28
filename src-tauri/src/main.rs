use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process::{Command};

#[derive(Debug, Serialize)]
struct Record {
    columns: HashMap<String, Value>,
}

fn csv_to_array_in_time(
    file_path: &str,
    start_time: i64,
    end_time: i64
) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    let headers = rdr.headers()?.clone();
    let mut records: Vec<Vec<String>> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let mut row_data: Vec<String> = Vec::new();
        let mut found_record = false;

        for (i, header) in headers.iter().enumerate() {
            let value = &record[i];
            let cleaned_header = header.trim().replace(")", "");

            if cleaned_header.is_empty() {
                continue;
            }

            if cleaned_header == "time" {
                let csv_time: i64 = value.parse()?;
                if csv_time < start_time || csv_time > end_time {
                    continue;
                } else {
                    found_record = true;
                }
            }

            if !value.trim().is_empty() {
                row_data.push(value.to_string());
            }
        }

        if found_record && !row_data.is_empty() {
            records.push(row_data);
        }
    }
    Ok(records)
}

#[tauri::command]
fn getdata1(starttime: i64, endtime: i64) -> Result<Vec<Vec<String>>, String> {
    let file_path = "bf.csv";
    let records = csv_to_array_in_time(&file_path, starttime, endtime)
        .map_err(|e| e.to_string())?;
    Ok(records)
}

#[tauri::command]
fn getdata2(starttime: i64, endtime: i64) -> Result<Vec<Vec<String>>, String> {
    let file_path = "bs.csv";
    let records = csv_to_array_in_time(&file_path, starttime, endtime)
        .map_err(|e| e.to_string())?;
    Ok(records)
}


fn empty_csv_except_first_row(file_path: &str) -> io::Result<()> {
    let path = Path::new(file_path);
    
    // Open the file in read mode
    let file = OpenOptions::new().read(true).open(&path)?;
    let reader = io::BufReader::new(file);
    
    // Read the first line (header)
    let first_line = reader.lines().next().unwrap_or(Ok(String::new()))?;
    
    // Open the file in write mode to truncate it
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)?;
    
    // Write the first line back to the file
    writeln!(file, "{}", first_line)?;
    
    Ok(())
}

fn main() {

        let files = ["bs.csv", "bf.csv"];
        
        for file in &files {
            match empty_csv_except_first_row(file) {
                Ok(()) => println!("Successfully emptied the CSV file: {}", file),
                Err(e) => eprintln!("Failed to empty the CSV file {}: {}", file, e),
            }
        }

        let second_project_executable = "/home/odin/Code/Tele/arduino_read/target/release/arduino_read"; // Adjust the path as neededcd

        // Start the second project in the background
        let mut child = Command::new(second_project_executable)
            .spawn()
            .expect("Failed to start the second project");
    
        println!("Second proje started with PID: {}", child.id());
    
        // You can continue with other tasks in the first project
        // ...
    
        // Optionally wait for the second project to finish (if needed)
        // let _ = child.wait().expect("Child process wasn't running");





    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![getdata1, getdata2])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
