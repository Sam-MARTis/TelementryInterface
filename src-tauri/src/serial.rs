use serial::prelude::*;
use std::io::prelude::*;
use std::time::Duration;
use std::io;
use serde_json;
use serde::Deserialize;

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
struct GpsData {
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





fn main() -> io::Result<()> {
    // Define the port name and configuration
    // let port_name = "/dev/ttyUSB0"; // Adjust based on your OS (e.g., COM3 for Windows)
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
    let mut serial_buf: [u8; 2048] = [0; 2048]; // Buffer to read chunks of data

    loop {
        match port.read(&mut serial_buf) {
            Ok(bytes_read) if bytes_read > 0 => {
                // Append the received data to the buffer
                buffer.push_str(&String::from_utf8_lossy(&serial_buf[..bytes_read]));
                
                // Check if buffer contains valid JSON (ending with '}')
                if buffer.trim().ends_with('}') {
                    // Determine which type of data it is
                    if buffer.contains("lat(deg)") {
                        // Try to parse the GPS data
                        match serde_json::from_str::<GpsData>(&buffer) {
                            Ok(parsed_data) => {
                                println!("Parsed GPS Data: {:?}", parsed_data);
                            }
                            Err(e) => {
                                eprintln!("Failed to parse GPS JSON: {:?}", e);
                            }
                        }
                    } else if buffer.contains("state") {
                        // Try to parse the sensor data
                        match serde_json::from_str::<SensorData>(&buffer) {
                            Ok(parsed_data) => {
                                println!("Parsed Sensor Data: {:?}", parsed_data);
                            }
                            Err(e) => {
                                eprintln!("Failed to parse Sensor JSON: {:?}", e);
                            }
                        }
                    } else {
                        eprintln!("Unknown data type or malformed JSON");
                    }
                    buffer.clear(); // Clear buffer for next message
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
