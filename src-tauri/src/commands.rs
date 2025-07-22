use crate::calculator::{calculate_days_between, compute_interest};
use crate::db::{delete_state_rate, get_all_states, get_state_rate, update_state_rate};
use crate::models::{CalcRequest, CalcResponse, StateRate};
use crate::rate_fetcher::{get_fallback_federal_rate, get_federal_rate};
use chrono::NaiveDate;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn calculate(
    app: AppHandle,
    request: CalcRequest,
) -> Result<CalcResponse, String> {
    // Parse dates
    let judgment_date = NaiveDate::parse_from_str(&request.judgment_date, "%Y-%m-%d")
        .map_err(|e| format!("Invalid judgment date: {}", e))?;
    let from_date = NaiveDate::parse_from_str(&request.from_date, "%Y-%m-%d")
        .map_err(|e| format!("Invalid from date: {}", e))?;
    let to_date = NaiveDate::parse_from_str(&request.to_date, "%Y-%m-%d")
        .map_err(|e| format!("Invalid to date: {}", e))?;

    // Validate date range
    if from_date > to_date {
        return Err("From date must be before or equal to end date".to_string());
    }

    // Get rate based on federal or state
    let (rate, rate_source) = if request.is_federal {
        // Fetch federal rate from FRED API
        let api_key = get_api_key(&app)?;
        match get_federal_rate(judgment_date, &api_key) {
            Ok(rate) => (rate, "Federal Rate (1-Year Treasury)".to_string()),
            Err(e) => {
                log::warn!("Failed to fetch federal rate: {}", e);
                (
                    get_fallback_federal_rate(),
                    "Federal Rate (Cached/Default)".to_string(),
                )
            }
        }
    } else {
        // Get state rate from database
        match get_state_rate(&app, &request.state) {
            Ok(Some(state_rate)) => {
                if state_rate.is_variable {
                    // Handle variable rates (e.g., Federal + X%)
                    let api_key = get_api_key(&app)?;
                    let base_rate = match get_federal_rate(judgment_date, &api_key) {
                        Ok(r) => r,
                        Err(_) => get_fallback_federal_rate(),
                    };
                    let final_rate = base_rate + (state_rate.plus_percentage / 100.0);
                    (
                        final_rate,
                        format!("{} (Variable: Federal + {}%)", state_rate.state, state_rate.plus_percentage),
                    )
                } else {
                    (
                        state_rate.rate / 100.0, // Convert percentage to decimal
                        format!("{} (Fixed: {}%)", state_rate.state, state_rate.rate),
                    )
                }
            }
            Ok(None) => return Err(format!("State '{}' not found in database", request.state)),
            Err(e) => return Err(format!("Database error: {}", e)),
        }
    };

    // Calculate interest
    let days = calculate_days_between(from_date, to_date);
    let interest_amount = compute_interest(request.amount, rate, days);
    let total_amount = request.amount + interest_amount;

    Ok(CalcResponse {
        rate: (rate * 100.0), // Convert back to percentage for display
        days,
        interest_amount,
        total_amount,
        rate_source,
        disclaimer: "This is an estimate only. Please consult legal advice for accurate calculations. Rates may need manual updates.".to_string(),
    })
}

#[tauri::command]
pub fn get_all_state_rates(app: AppHandle) -> Result<Vec<StateRate>, String> {
    get_all_states(&app).map_err(|e| format!("Failed to fetch state rates: {}", e))
}

#[tauri::command]
pub fn update_state_rate_command(
    app: AppHandle,
    state_rate: StateRate,
) -> Result<(), String> {
    update_state_rate(&app, &state_rate)
        .map_err(|e| format!("Failed to update state rate: {}", e))
}

#[tauri::command]
pub fn delete_state_rate_command(app: AppHandle, id: i32) -> Result<(), String> {
    delete_state_rate(&app, id).map_err(|e| format!("Failed to delete state rate: {}", e))
}

#[tauri::command]
pub fn set_api_key(app: AppHandle, api_key: String) -> Result<(), String> {
    let store_path = app
        .path()
        .app_data_dir()
        .unwrap()
        .join("config.json");
    
    let config = serde_json::json!({
        "fred_api_key": api_key
    });
    
    std::fs::write(&store_path, serde_json::to_string_pretty(&config).unwrap())
        .map_err(|e| format!("Failed to save API key: {}", e))?;
    
    Ok(())
}

#[tauri::command]
pub fn get_api_key_configured(app: AppHandle) -> Result<bool, String> {
    match get_api_key(&app) {
        Ok(key) => Ok(!key.is_empty()),
        Err(_) => Ok(false),
    }
}

// Helper function to get API key from config
fn get_api_key(app: &AppHandle) -> Result<String, String> {
    let store_path = app
        .path()
        .app_data_dir()
        .unwrap()
        .join("config.json");
    
    if !store_path.exists() {
        return Err("API key not configured. Please set it in the settings.".to_string());
    }
    
    let content = std::fs::read_to_string(&store_path)
        .map_err(|e| format!("Failed to read config: {}", e))?;
    
    let config: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse config: {}", e))?;
    
    match config.get("fred_api_key") {
        Some(serde_json::Value::String(key)) => Ok(key.clone()),
        _ => Err("API key not found in config".to_string()),
    }
}

#[tauri::command]
pub fn validate_api_key_command(api_key: String) -> Result<bool, String> {
    use crate::rate_fetcher::validate_api_key;
    
    match validate_api_key(&api_key) {
        Ok(valid) => Ok(valid),
        Err(e) => Err(format!("Failed to validate API key: {}", e)),
    }
}
