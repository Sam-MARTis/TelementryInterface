use std::error::Error;
use serde::{Serialize, Deserialize};  // Import Deserialize

// SensorData struct definition
#[derive(Serialize, Deserialize)]  // Add Deserialize here
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

// GpsData struct definition
#[derive(Serialize, Deserialize)]  // Add Deserialize here
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
