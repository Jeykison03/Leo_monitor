use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use std::sync::{Arc, Mutex};
use dotenv::dotenv;
use log::info;
use std::thread;

mod db;
mod routes;
mod arduino_serial;
mod models;

pub struct AppState {
    pub db: db::MongoRepo,
    pub serial_data: Arc<Mutex<models::SharedHeartRate>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Starting Heart Rate Monitor Backend...");

    let db = db::MongoRepo::init().await;
    
    let shared_data = Arc::new(Mutex::new(models::SharedHeartRate {
        current_bpm: 70, // Start with a realistic value
        current_raw: 512,
        last_updated: None
    }));

    let serial_data_clone = shared_data.clone();
    
    let db_clone = db.clone();

    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            arduino_serial::run_serial_listener(serial_data_clone, db_clone).await;
        });
    });

    let app_state = web::Data::new(AppState {
        db,
        serial_data: shared_data.clone(),
    });

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(app_state.clone())
            .configure(routes::config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
