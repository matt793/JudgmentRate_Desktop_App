use crate::models::{FredResponse, FredObservation};
use chrono::{Datelike, Duration, NaiveDate};
use thiserror::Error;
use reqwest::blocking::Client;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("API request failed: {0}")]
    RequestError(String),
    #[error("Failed to parse response: {0}")]
    ParseError(String),
    #[error("Failed to parse date: {0}")]
    DateError(#[from] chrono::ParseError),
    #[error("No valid observations found")]
    NoValidObservations,
    #[error("API key not configured")]
    ApiKeyMissing,
}

pub fn get_federal_rate(
    judgment_date: NaiveDate,
    api_key: &str,
) -> Result<f64, FetchError> {
    if api_key.is_empty() {
        return Err(FetchError::ApiKeyMissing);
    }

    // Calculate the preceding week (Monday to Sunday)
    let (start_date, end_date) = get_preceding_week(judgment_date);
    
    // Fetch data from FRED API
    let observations = fetch_fred_data(api_key, start_date, end_date)?;
    
    // Calculate average rate
    calculate_average_rate(observations)
}

// Calculates the Monday to Sunday of the week before the judgment date
fn get_preceding_week(judgment_date: NaiveDate) -> (NaiveDate, NaiveDate) {
    let days_since_monday = judgment_date.weekday().num_days_from_monday();
    let last_monday = judgment_date - Duration::days(days_since_monday as i64 + 7);
    let last_sunday = last_monday + Duration::days(6);
    
    (last_monday, last_sunday)
}

fn fetch_fred_data(
    api_key: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<FredObservation>, FetchError> {
    let series_id = "DGS1"; // 1-Year Treasury Constant Maturity Rate
    let url = format!(
        "https://api.stlouisfed.org/fred/series/observations?series_id={}&api_key={}&file_type=json&observation_start={}&observation_end={}",
        series_id,
        api_key,
        start_date.format("%Y-%m-%d"),
        end_date.format("%Y-%m-%d")
    );
    
    let client = Client::new();
    let response = client
        .get(&url)
        .send()
        .map_err(|e| FetchError::RequestError(e.to_string()))?;
    
    let fred_response: FredResponse = response
        .json()
        .map_err(|e| FetchError::ParseError(e.to_string()))?;
    
    Ok(fred_response.observations)
}

fn calculate_average_rate(observations: Vec<FredObservation>) -> Result<f64, FetchError> {
    let valid_rates: Vec<f64> = observations
        .iter()
        .filter(|obs| obs.value != ".")
        .filter_map(|obs| obs.value.parse::<f64>().ok())
        .collect();
    
    if valid_rates.is_empty() {
        return Err(FetchError::NoValidObservations);
    }
    
    let sum: f64 = valid_rates.iter().sum();
    let average = sum / valid_rates.len() as f64;
    
    // Convert from percentage to decimal
    Ok(average / 100.0)
}

// Helper function to get a cached/default rate if API fails
pub fn get_fallback_federal_rate() -> f64 {
    // Default to 5% if we can't fetch the actual rate
    0.05
}

// Function to validate API key by making a test request
pub fn validate_api_key(api_key: &str) -> Result<bool, FetchError> {
    if api_key.is_empty() {
        return Ok(false);
    }
    
    // Make a simple test request to check if the API key is valid
    let test_url = format!(
        "https://api.stlouisfed.org/fred/series?series_id=DGS1&api_key={}&file_type=json",
        api_key
    );
    
    let client = Client::new();
    match client.get(&test_url).send() {
        Ok(response) => {
            // Check if we got a successful response
            Ok(response.status().is_success())
        }
        Err(e) => {
            // Check if it's a client error (4xx) which would indicate invalid API key
            if let Some(status) = e.status() {
                if status.as_u16() == 400 || status.as_u16() == 403 {
                    Ok(false)
                } else {
                    Err(FetchError::RequestError(format!("HTTP error: {}", status)))
                }
            } else {
                // Network or other errors
                Err(FetchError::RequestError(format!("Request failed: {}", e)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_preceding_week() {
        // Test case: Wednesday Jan 10, 2024
        let judgment_date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        let (start, end) = get_preceding_week(judgment_date);
        
        // Should return Monday Jan 1 to Sunday Jan 7
        assert_eq!(start, NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
        assert_eq!(end, NaiveDate::from_ymd_opt(2024, 1, 7).unwrap());
    }
}
