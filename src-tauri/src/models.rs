use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateRate {
    pub id: i32,
    pub state: String,
    pub rate: f64,
    pub is_variable: bool,
    pub plus_percentage: f64,
    pub update_frequency: String,
    pub last_update: String,
    pub notes: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalcRequest {
    pub judgment_date: String,
    pub is_federal: bool,
    pub state: String,
    pub amount: f64,
    pub from_date: String,
    pub to_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalcResponse {
    pub rate: f64,
    pub days: i64,
    pub interest_amount: f64,
    pub total_amount: f64,
    pub rate_source: String,
    pub disclaimer: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FredResponse {
    pub realtime_start: String,
    pub realtime_end: String,
    pub observations: Vec<FredObservation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FredObservation {
    pub realtime_start: String,
    pub realtime_end: String,
    pub date: String,
    pub value: String,
}

impl StateRate {
    pub fn new(
        state: String,
        rate: f64,
        is_variable: bool,
        plus_percentage: f64,
        update_frequency: String,
        notes: String,
    ) -> Self {
        Self {
            id: 0,
            state,
            rate,
            is_variable,
            plus_percentage,
            update_frequency,
            last_update: chrono::Local::now().format("%Y-%m-%d").to_string(),
            notes,
        }
    }
}
