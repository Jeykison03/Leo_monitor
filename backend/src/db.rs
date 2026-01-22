use std::env;
use mongodb::{Client, Collection};
use mongodb::bson::doc;
use chrono::{DateTime, Utc};
use futures::stream::TryStreamExt;
use log::{error, info};
use crate::models::FhirObservation;

#[derive(Clone)]
pub struct MongoRepo {
    col: Collection<FhirObservation>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        let uri = env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
        
        info!("Connecting to MongoDB at {}", uri);
        
        let client = Client::with_uri_str(&uri).await.expect("Failed to connect to MongoDB");
        let db = client.database("heart_rate_monitor");
        let col: Collection<FhirObservation> = db.collection("observations");

        MongoRepo { col }
    }

    pub async fn insert_heart_rate(&self, observation: FhirObservation) {
        match self.col.insert_one(observation).await {
            Ok(_) => {},
            Err(e) => error!("Error saving to MongoDB: {}", e),
        }
    }

    pub async fn get_data_in_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<FhirObservation> {
        let filter = doc! {
            "effectiveDateTime": {
                "$gte": start.to_rfc3339(),
                "$lte": end.to_rfc3339()
            }
        };
        
        let mut cursor = match self.col.find(filter).await {
            Ok(c) => c,
            Err(e) => {
                error!("DB Query Error: {}", e);
                return vec![];
            }
        };
        
        let mut results = Vec::new();
        while let Ok(Some(doc)) = cursor.try_next().await {
            results.push(doc);
        }
        results
    }
}
