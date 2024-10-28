use std::error::Error;
use serde::{Serialize, Deserialize};  // Import Deserialize
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{json, Value};
use serial::prelude::*;
use std::io;
use std::io::prelude::*;
use std::time::Duration;

#[derive(Deserialize, Debug)]
struct SensorData {
    r#type: String,
    time: u64,
    state: u8,
    temperature: f32,
    #[serde(rename = "alt")]
    lat_min: f32,
    #[serde(rename = "ram_diff")]
    ram_diff: f32,
    #[serde(rename = "bno_x")]
    bno_x: f32,
    #[serde(rename = "bno_y")]
    bno_y: f32,
    #[serde(rename = "bno_z")]
    bno_z: f32,
    #[serde(rename = "high_x")]
    high_x: f32,
    #[serde(rename = "high_y")]
    high_y: f32,
    #[serde(rename = "high_z")]
    high_z: f32,
    #[serde(rename = "gyro_x")]
    gyro_x: f32,
    #[serde(rename = "gyro_y")]
    gyro_y: f32,
    #[serde(rename = "gyro_z")]
    gyro_z: f32,
}

#[derive(Deserialize, Debug)]
struct GpsDataTemp {
    r#type: String,
    
    #[serde(rename = "lat(deg)")]
    lat_deg: u32,
    #[serde(rename = "lat(min)")]
    lat_min: String,
    #[serde(rename = "lat(sec)")]
    lat_sec: f64,
    #[serde(rename = "lat(N/W)")]
    lat_nw: u32,
    #[serde(rename = "lon(deg)")]
    lon_deg: u32,
    #[serde(rename = "lon(min)")]
    lon_min: u32,
    #[serde(rename = "lon(sec)")]
    lon_sec: f64,
    #[serde(rename = "lon(E/W)")]
    lon_ew: u32,
    v_horizontal: f32,
    course: f32,
    hdop: f32,
    vdop: f32,
    #[serde(rename = "type2")]
    type2: String,
    #[serde(rename = "alt(ABL)")]
    alt_abl: f32,
    fix_time_since_start: u32,
    time_since_fix: u32,
}

#[derive(Deserialize, Debug)]
struct GpsData {
    r#type: String,
    time: u64,

    #[serde(rename = "lat(deg)")]
    lat_deg: u32,
    #[serde(rename = "lat(min)")]
    lat_min: String,
    #[serde(rename = "lat(sec)")]
    lat_sec: f64,
    #[serde(rename = "lat(N/W)")]
    lat_nw: u32,
    #[serde(rename = "lon(deg)")]
    lon_deg: u32,
    #[serde(rename = "lon(min)")]
    lon_min: u32,
    #[serde(rename = "lon(sec)")]
    lon_sec: f64,
    #[serde(rename = "lon(E/W)")]
    lon_ew: u32,
    v_horizontal: f32,
    course: f32,
    hdop: f32,
    vdop: f32,
    #[serde(rename = "type2")]
    type2: String,
    #[serde(rename = "alt(ABL)")]
    alt_abl: f32,
    fix_time_since_start: u32,
    time_since_fix: u32,
}

fn main_samanth() -> io::Result<()> {
    // Define the port name and configuration
    let port_name = "/dev/ttyACM0"; // Adjust based on your OS (e.g., COM3 for Windows)

    // Open the serial port
    let mut port = serial::open(port_name)?;

    // Configure the port settings
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::BaudRate::Baud9600)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })?;

    // Set a read timeout
    port.set_timeout(Duration::from_secs(1))?;

    println!("Serial port open and configured. Listening for data...");

    // Buffer to store incoming serial data
    let mut buffer = String::new();
    let mut serial_buf: [u8; 1024] = [0; 1024]; // Buffer to read chunks of data

    // Variable to store the latest sensor time
    let mut latest_sensor_time: Option<u64> = None;

    loop {
        match port.read(&mut serial_buf) {
            Ok(bytes_read) if bytes_read > 0 => {
                // Append the received data to the buffer
                buffer.push_str(&String::from_utf8_lossy(&serial_buf[..bytes_read]));

                // Process complete JSON objects from the buffer
                while let Some(pos) = buffer.find('\n') {
                    // Extract the complete line as a JSON string
                    let json_line = buffer[..pos].trim().to_string();
                    buffer = buffer[pos + 1..].to_string(); // Remove the processed line

                    // Determine which type of data it is
                    if json_line.contains("state") {
                        // Try to parse the sensor data
                        match serde_json::from_str::<SensorData>(&json_line) {
                            Ok(sensor_data) => {
                                println!("Parsed Sensor Data: {:?}", sensor_data);

                                // Store the sensor time in the variable
                                latest_sensor_time = Some(sensor_data.time);
                            }
                            Err(e) => {
                                eprintln!("Failed to parse Sensor JSON: {:?}", e);
                            }
                        }
                    } else if json_line.contains("lat(deg)") {
                        // Try to parse the GPS data
                        match serde_json::from_str::<GpsDataTemp>(&json_line) {
                            Ok(mut gps_data) => {
                                println!("Parsed GPS Data: {:?}", gps_data);

                                // Check if we have a valid latest sensor time to add
                                if let Some(sensor_time) = latest_sensor_time {
                                    // Add the sensor time to the GPS data
                                    let mut gps_json: Value = serde_json::from_str(&json_line)?;

                                    // Add the sensor_time field
                                    gps_json["time"] = json!(sensor_time);

                                    // Convert the modified gps_json (Value) back into a string
                                    let gps_json_str = serde_json::to_string(&gps_json)?;

                                    // Deserialize the modified JSON string into GpsData2 struct
                                    let gps_data2: GpsData = serde_json::from_str(&gps_json_str)?;

                                    // Print out the final struct
                                    println!("Deserialized GpsData2: {:?}", gps_data2);
                                } else {
                                    println!("No sensor time available to add to GPS data");
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to parse GPS JSON: {:?}", e);
                            }
                        }
                    } else {
                        eprintln!("Unknown data type or malformed JSON");
                    }
                }
            }
            Ok(_) => {
                // If no data is received, just continue the loop
                continue;
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                // If a timeout occurs, continue to check for new data
                continue;
            }
            Err(e) => {
                // Handle other errors
                eprintln!("Failed to read from serial port: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}


impl SensorData {
    // Convert SensorData instance to Vec<String>
    fn to_string_vector(&self) -> Vec<String> {
        vec![
            self.r#type.clone(),
            self.time.to_string(),
            self.state.to_string(),
            self.temperature.to_string(),
            self.lat_min.to_string(),
            self.ram_diff.to_string(),
            self.bno_x.to_string(),
            self.bno_y.to_string(),
            self.bno_z.to_string(),
            self.high_x.to_string(),
            self.high_y.to_string(),
            self.high_z.to_string(),
            self.gyro_x.to_string(),
            self.gyro_y.to_string(),
            self.gyro_z.to_string(),
        ]
    }
}

impl GpsData {
    // Convert GpsData instance to Vec<String>
    fn to_string_vector(&self) -> Vec<String> {
        vec![
            self.r#type.clone(),
            self.time.to_string(),
            self.lat_deg.to_string(),
            self.lat_min.clone(),
            self.lat_sec.to_string(),
            self.lat_nw.to_string(),
            self.lon_deg.to_string(),
            self.lon_min.to_string(),
            self.lon_sec.to_string(),
            self.lon_ew.to_string(),
            self.v_horizontal.to_string(),
            self.course.to_string(),
            self.hdop.to_string(),
            self.vdop.to_string(),
            self.type2.clone(),
            self.alt_abl.to_string(),
            self.fix_time_since_start.to_string(),
            self.time_since_fix.to_string(),
        ]
    }
}

// Function to filter and convert SensorData based on time
fn filter_sensor_data_by_time(
    data: &[SensorData],
    start_time: u64,
    end_time: u64,
) -> Result<Vec<Vec<String>>, String> {
    let filtered: Vec<Vec<String>> = data.iter()
        .filter(|&d| d.time >= start_time && d.time <= end_time)
        .map(|d| d.to_string_vector())
        .collect();

    Ok(filtered)
}

// Function to filter and convert GpsData based on time
fn filter_gps_data_by_time(
    data: &[GpsData],
    start_time: u64,
    end_time: u64,
) -> Result<Vec<Vec<String>>, String> {
    let filtered: Vec<Vec<String>> = data.iter()
        .filter(|&d| d.time >= start_time && d.time <= end_time)
        .map(|d| d.to_string_vector())
        .collect();

    Ok(filtered)
}

// Tauri command to filter sensor data
#[tauri::command]
fn filter_sensor_data(start_time: u64, end_time: u64, sensor_data: Vec<SensorData>) -> Result<Vec<Vec<String>>, String> {
    filter_sensor_data_by_time(&sensor_data, start_time, end_time)
}

// Tauri command to filter GPS data
#[tauri::command]
fn filter_gps_data(start_time: u64, end_time: u64, gps_data: Vec<GpsData>) -> Result<Vec<Vec<String>>, String> {
    filter_gps_data_by_time(&gps_data, start_time, end_time)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![filter_sensor_data, filter_gps_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
