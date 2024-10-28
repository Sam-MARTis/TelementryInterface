use std::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serial::prelude::*;
use std::io;
use std::io::prelude::*;
use std::time::Duration;
// use tokio_serial::{Serial, SerialPortBuilderExt};
use tokio::io::{AsyncBufReadExt, BufReader};
// use std::time::Duration;

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
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

// Main function to collect sensor and GPS data from the serial port
fn main_samanth(
    sensor_data_array: &mut Vec<SensorData>,
    gps_data_array: &mut Vec<GpsData>,
) -> io::Result<()> {
    let port_name = "/dev/ttyACM0";

    let mut port = serial::open(port_name)?;
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::BaudRate::Baud9600)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })?;
    port.set_timeout(Duration::from_secs(1))?;

    println!("Serial port open and configured. Listening for data...");

    let mut buffer = String::new();
    let mut serial_buf: [u8; 1024] = [0; 1024];
    let mut latest_sensor_time: Option<u64> = None;

    loop {
        match port.read(&mut serial_buf) {
            Ok(bytes_read) if bytes_read > 0 => {
                buffer.push_str(&String::from_utf8_lossy(&serial_buf[..bytes_read]));

                while let Some(pos) = buffer.find('\n') {
                    let json_line = buffer[..pos].trim().to_string();
                    buffer = buffer[pos + 1..].to_string();

                    if json_line.contains("state") {
                        match serde_json::from_str::<SensorData>(&json_line) {
                            Ok(sensor_data) => {
                                // println!("Parsed Sensor Data: {:?}", sensor_data);
                                latest_sensor_time = Some(sensor_data.time);
                                sensor_data_array.push(sensor_data);
                            }
                            Err(e) => {
                                eprintln!("Failed to parse Sensor JSON: {:?}", e);
                            }
                        }
                    } else if json_line.contains("lat(deg)") {
                        if let Some(sensor_time) = latest_sensor_time {
                            let mut gps_json: Value = serde_json::from_str(&json_line)?;
                            gps_json["time"] = json!(sensor_time);

                            let gps_json_str = serde_json::to_string(&gps_json)?;
                            match serde_json::from_str::<GpsData>(&gps_json_str) {
                                Ok(gps_data) => {
                                    // println!("Parsed GPS Data: {:?}", gps_data);
                                    gps_data_array.push(gps_data);
                                }
                                Err(e) => {
                                    eprintln!("Failed to parse GPS JSON: {:?}", e);
                                }
                            }
                        } else {
                            println!("No sensor time available to add to GPS data");
                        }
                    } else {
                        eprintln!("Unknown data type or malformed JSON");
                    }
                }
            }
            Ok(_) => continue,
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => continue,
            Err(e) => {
                eprintln!("Failed to read from serial port: {:?}", e);
                break;
            }
        }
    }

    Ok(())
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
fn getdata1(start_time: u64, end_time: u64, gps_data: Vec<GpsData>) -> Result<Vec<Vec<String>>, String> {
    filter_gps_data_by_time(&gps_data, start_time, end_time)
}



#[tokio::main]
async fn main() {
    // Initialize the data arrays
    let mut sensor_data_array: Vec<SensorData> = Vec::new();
    let mut gps_data_array: Vec<GpsData> = Vec::new();

    // Move the data arrays into the async block
    let result = tokio::spawn(async move {
        main_samanth(&mut sensor_data_array, &mut gps_data_array)
    }).await;

    if let Err(e) = result {
        eprintln!("Error in main_samanth: {:?}", e);
    }

    // println!("All Sensor Data: {:?}", sensor_data_array);
    // println!("All GPS Data: {:?}", gps_data_array);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![filter_sensor_data, getdata1])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}