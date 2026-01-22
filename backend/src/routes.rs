use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder};
use actix_ws::Message;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time::interval;
use log::{error, info};
use crate::models::SharedHeartRate;
use crate::AppState;
use futures_util::StreamExt as _;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check)
       .service(ws_heart_rate)
       .service(get_history);
}

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Heart Rate Monitor Backend is Running")
}

#[derive(serde::Deserialize)]
struct StatsQuery {
    start: Option<String>,
    end: Option<String>,
}

#[get("/api/stats")]
async fn get_history(data: web::Data<AppState>, query: web::Query<StatsQuery>) -> impl Responder {
    let end = chrono::Utc::now();
    let start = end - chrono::Duration::minutes(10);
    
    let start_time = query.start.as_ref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok().map(|dt| dt.with_timezone(&chrono::Utc)))
        .unwrap_or(start);
        
    let end_time = query.end.as_ref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok().map(|dt| dt.with_timezone(&chrono::Utc)))
        .unwrap_or(end);

    let records = data.db.get_data_in_range(start_time, end_time).await;
    
    let count = records.len();
    let (min, max, avg) = if count > 0 {
        let values: Vec<i32> = records.iter().map(|r| r.value_quantity.value as i32).collect();
        let min = *values.iter().min().unwrap();
        let max = *values.iter().max().unwrap();
        let sum: i32 = values.iter().sum();
        let avg = sum / count as i32;
        (min, max, avg)
    } else {
        (0, 0, 0)
    };
    
    HttpResponse::Ok().json(serde_json::json!({
        "start": start_time.to_rfc3339(),
        "end": end_time.to_rfc3339(),
        "count": count,
        "min": min,
        "max": max,
        "avg": avg
    }))
}

#[get("/ws/heart_rate")]
async fn ws_heart_rate(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let (res, session, stream) = actix_ws::handle(&req, stream)?;

    let shared_data = data.serial_data.clone();
    
    actix_web::rt::spawn(async move {
        run_ws(session, stream, shared_data).await;
    });

    Ok(res)
}

async fn run_ws(
    mut session: actix_ws::Session,
    mut stream: actix_ws::MessageStream,
    shared_data: Arc<Mutex<SharedHeartRate>>,
) {
    info!("New WebSocket connected");
    
    let mut last_heartbeat = Instant::now();
    let mut ticker = interval(Duration::from_millis(20));

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                let (current_bpm, current_raw, last_updated) = {
                    let state = shared_data.lock().unwrap();
                    (state.current_bpm, state.current_raw, state.last_updated)
                };
                
                let payload = serde_json::json!({
                    "type": "BIO_DATA",
                    "bpm": current_bpm,
                    "raw": current_raw,
                    "last_beat": last_updated,
                    "timestamp": chrono::Utc::now().to_rfc3339()
                });
                
                if let Err(e) = session.text(payload.to_string()).await {
                    error!("WS Send Error: {}", e);
                    break;
                }
            }

            msg = stream.next() => {
                match msg {
                    Some(Ok(Message::Ping(bytes))) => {
                        last_heartbeat = Instant::now();
                        if session.pong(&bytes).await.is_err() { break; }
                    }
                    Some(Ok(Message::Close(_))) => {
                        info!("WS Client disconnected");
                        break;
                    }
                    Some(Err(e)) => {
                        error!("WS Error: {}", e);
                        break;
                    }
                    None => break,
                    _ => {}
                }
            }
        }
    }
}
