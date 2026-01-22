use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HeartRateRecord {
    pub value: i32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Default)]
pub struct SharedHeartRate {
    pub current_bpm: i32,
    pub current_raw: i32,
    pub last_updated: Option<DateTime<Utc>>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FhirObservation {
    pub resource_type: String,
    pub status: String,
    pub code: FhirCodeableConcept,
    pub subject: FhirReference,
    pub effective_date_time: String,
    pub value_quantity: FhirQuantity,
    pub device: Option<FhirReference>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FhirCodeableConcept {
    pub coding: Vec<FhirCoding>,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FhirCoding {
    pub system: String,
    pub code: String,
    pub display: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FhirReference {
    pub reference: String,
    pub display: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FhirQuantity {
    pub value: f64,
    pub unit: String,
    pub system: String,
    pub code: String,
}

impl FhirObservation {
    pub fn new_heart_rate(bpm: i32, user_id: &str) -> Self {
        FhirObservation {
            resource_type: "Observation".to_string(),
            status: "final".to_string(),
            code: FhirCodeableConcept {
                coding: vec![FhirCoding {
                    system: "http://loinc.org".to_string(),
                    code: "8867-4".to_string(),
                    display: "Heart rate".to_string(),
                }],
                text: "Heart rate".to_string(),
            },
            subject: FhirReference {
                reference: format!("Patient/{}", user_id),
                display: Some("Student User".to_string()),
            },
            effective_date_time: Utc::now().to_rfc3339(),
            value_quantity: FhirQuantity {
                value: bpm as f64,
                unit: "beats/minute".to_string(),
                system: "http://unitsofmeasure.org".to_string(),
                code: "/min".to_string(),
            },
            device: Some(FhirReference {
                reference: "Device/arduino-uno-r3".to_string(),
                display: Some("Arduino Heart Rate Sensor".to_string()),
            }),
        }
    }
}
