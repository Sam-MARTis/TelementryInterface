use serde_json::Value;
use tauri::api::file;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use csv::ReaderBuilder;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Record {
    columns: HashMap<String, Value>,
}
fn csv_to_json_in_time(
    file_path: &str,
    start_time: i64,
    end_time: i64
)  -> Result<String, Box<dyn Error>>  {
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

            // Only add valid values (integers, floats, or non-empty strings)
            let parsed_value = if let Ok(int_value) = value.parse::<i64>() {
                Value::from(int_value) // Store as integer
            } else if let Ok(float_value) = value.parse::<f64>() {
                Value::from(float_value) // Store as float
            } else if !value.trim().is_empty() {
                Value::from(value.to_string()) // Store as non-empty string
            } else {
                continue; // Skip invalid or empty values
            };

            // Insert the valid value into the row data
            row_data.insert(header.to_string(), parsed_value);
        }

        // Add the filtered record to the result set if row_data is not empty and we've found a matching record
        if found_record && !row_data.is_empty() {
            records.push(Record { columns: row_data });
        }
    }

    // Convert records to JSON
    let json_data = serde_json::to_string_pretty(&records)?;
    Ok(json_data)
}
fn csv_to_array_in_time(
    file_path: &str,
    start_time: i64,
    end_time: i64
) -> Result<Vec<Record>, Box<dyn Error>> {
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
   
               // Clean up the header by trimming spaces and handling parentheses
               let cleaned_header = header.trim().replace(")", "");
   
               // Skip if the cleaned header is empty
               if cleaned_header.is_empty() {
                   continue;
               }
   
               // Check if the current header is the time column
               if cleaned_header == "time" {
                   // Parse the time value as an integer
                   let csv_time: i64 = value.parse()?;
   
                   // Skip rows where the time doesn't match the filter
                   if csv_time < start_time || csv_time > end_time {
                       continue;
                   } else {
                       found_record = true; // Mark as found if the time matches the range
                   }
               }
   
               // Only add valid values (integers, floats, or non-empty strings)
               let parsed_value = if let Ok(int_value) = value.parse::<i64>() {
                   Value::from(int_value) // Store as integer
               } else if let Ok(float_value) = value.parse::<f64>() {
                   Value::from(float_value) // Store as float
               } else if !value.trim().is_empty() {
                   Value::from(value.to_string()) // Store as non-empty string
               } else {
                   continue; // Skip invalid or empty values
               };
   
               // Insert the valid value into the row data
               row_data.insert(cleaned_header.to_string(), parsed_value);
           }
   
           // Add the filtered record to the result set if row_data is not empty and we've found a matching record
           if found_record && !row_data.is_empty() {
               records.push(Record { columns: row_data });
           }
       }
       
       // Return the records vector
       Ok(records)  // Remove the semicolon here
}
use tauri::command;

#[command]
fn getshit1(starttime: i64, endtime: i64) -> Result<Vec<Record>, String> {
    let file_path = "bf.csv";
    
    match csv_to_array_in_time(&file_path, starttime, endtime) {
        Ok(records) => Ok(records), // Directly return the vector of records
        Err(e) => Err(format!("Error: {}", e)), // Handle CSV reading errors
    }
}

fn getshit2(starttime: i64, endtime: i64) -> Result<Vec<Record>, String> {
    let file_path = "bs.csv";
    
    match csv_to_array_in_time(&file_path, starttime, endtime) {
        Ok(records) => Ok(records), // Directly return the vector of records
        Err(e) => Err(format!("Error: {}", e)), // Handle CSV reading errors
    }
}



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![getshit1])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}