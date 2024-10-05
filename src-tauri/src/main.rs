use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use serde::Serialize;

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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![getdata1])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
