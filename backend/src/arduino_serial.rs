use std::env;
use std::io::{self, BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use serialport::SerialPort;
use tokio::time::sleep;
use log::{info, warn, error};
use crate::models::{SharedHeartRate, FhirObservation};
use crate::db::MongoRepo;

pub async fn run_serial_listener(shared_state: Arc<Mutex<SharedHeartRate>>, db: MongoRepo) {
    let port_name = env::var("SERIAL_PORT").unwrap_or_else(|_| "COM3".to_string());
    let baud_rate = 9600;
    
    let simulation_mode = env::var("SIMULATION_MODE").unwrap_or_else(|_| "false".to_string()) == "true";

    if simulation_mode {
        info!("Starting SIMULATION MODE (No Arduino connected)");
        run_simulation(shared_state, db).await;
        return;
    }

    info!("Attempting to open Serial Port: {}", port_name);

    loop {
        match serialport::new(&port_name, baud_rate).timeout(Duration::from_millis(1000)).open() {
            Ok(port) => {
                info!("Serial port {} opened successfully!", port_name);
                read_from_port(port, &shared_state, &db).await;
                warn!("Serial port disconnected. Retrying in 5 seconds...");
            }
            Err(e) => {
                warn!("Failed to open serial port {}: {}. Retrying in 5s...", port_name, e);
            }
        }
        sleep(Duration::from_secs(5)).await;
    }
}

// State for BPM calculation (Global/Static would be easier, but we are in async)
// We will wrap this in a struct or just use a simple state machine in the loop.

struct SignalProcessor {
    last_beat_time: std::time::Instant,
    current_bpm: i32,
}

async fn read_from_port(port: Box<dyn SerialPort>, shared_state: &Arc<Mutex<SharedHeartRate>>, db: &MongoRepo) {
    let mut reader = BufReader::new(port);
    let mut buffer = String::new();
    
    let mut last_beat = std::time::Instant::now();
    let threshold_high = 535;
    let threshold_low = 515;
    let mut pulse_triggered = false;

    loop {
        buffer.clear();
        match reader.read_line(&mut buffer) {
            Ok(n) if n > 0 => {
                let line = buffer.trim();
                
                if line.starts_with("RAW:") {
                    if let Ok(raw_val) = line["RAW:".len()..].parse::<i32>() {
                        
                        {
                            let mut state = shared_state.lock().unwrap();
                            state.current_raw = raw_val; 
                        }


                        if raw_val > threshold_high && !pulse_triggered {
                            pulse_triggered = true;
                            let now = std::time::Instant::now();
                            let delta = now.duration_since(last_beat).as_millis() as i32;
                            
                            if delta > 450 {
                                let bpm = 60000 / delta;
                                last_beat = now;
                                
                                if bpm > 40 && bpm < 200 {
                                     info!("Beat Detected! BPM: {}", bpm);
                                     process_heart_rate(bpm, shared_state, db).await;
                                }
                            }
                        }
                        
                        if raw_val < threshold_low {
                            pulse_triggered = false;
                        }
                    }
                }
            }
            Ok(_) => {
                // EOF or empty
                break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                tokio::task::yield_now().await; 
                continue;
            }
            Err(e) => {
                error!("Serial Read Error: {}", e);
                break;
            }
        }
        
        tokio::task::yield_now().await;
    }
}

async fn run_simulation(shared_state: Arc<Mutex<SharedHeartRate>>, db: MongoRepo) {
    let mut rng_val = 70;
    loop {
        let change = (rand::random::<i32>() % 7) - 3;
        rng_val += change;
        if rng_val < 50 { rng_val = 55; }
        if rng_val > 150 { rng_val = 145; }
        
        if rand::random::<u8>() > 250 {
            rng_val = 110;
        }

        process_heart_rate(rng_val, &shared_state, &db).await;
        sleep(Duration::from_millis(1000)).await;
    }
}

async fn process_heart_rate(bpm: i32, shared_state: &Arc<Mutex<SharedHeartRate>>, db: &MongoRepo) {
    {
        let mut state = shared_state.lock().unwrap();
        state.current_bpm = bpm;
        state.last_updated = Some(chrono::Utc::now());
    }
    
    if bpm > 100 {
        warn!("High Heart Rate Detected: {} BPM", bpm);
    }

    let fhir_doc = FhirObservation::new_heart_rate(bpm, "user-123");
    db.insert_heart_rate(fhir_doc).await;
    
    info!("Processed HR: {}", bpm);
}
