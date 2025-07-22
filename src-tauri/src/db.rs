use crate::models::StateRate;
use rusqlite::{Connection, OptionalExtension, Result, params};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub fn get_db_path(app: &AppHandle) -> PathBuf {
    let mut path = app
        .path()
        .app_data_dir()
        .expect("Failed to get app data dir");
    std::fs::create_dir_all(&path).expect("Failed to create app data dir");
    path.push("rates.db");
    path
}

pub fn init_db(app: &AppHandle) -> Result<()> {
    let db_path = get_db_path(app);
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS state_rates (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            state TEXT NOT NULL UNIQUE,
            rate REAL NOT NULL,
            is_variable INTEGER NOT NULL,
            plus_percentage REAL NOT NULL,
            update_frequency TEXT NOT NULL,
            last_update TEXT NOT NULL,
            notes TEXT
        )",
        [],
    )?;

    // Check if we need to seed initial data
    let count: i32 = conn.query_row("SELECT COUNT(*) FROM state_rates", [], |row| row.get(0))?;
    
    if count == 0 {
        seed_initial_data(&conn)?;
    }

    Ok(())
}

fn seed_initial_data(conn: &Connection) -> Result<()> {
    let initial_states = vec![
        // Federal rate is variable
        StateRate::new("Federal".to_string(), 0.0, true, 0.0, "Weekly".to_string(), "Based on 1-year Treasury yield".to_string()),
        
        // State rates (simplified - in production, research actual rates)
        StateRate::new("Alabama".to_string(), 7.5, false, 0.0, "Annual".to_string(), "7.5% per annum".to_string()),
        StateRate::new("Alaska".to_string(), 3.0, true, 0.0, "Annual".to_string(), "Federal rate + 3%".to_string()),
        StateRate::new("Arizona".to_string(), 10.0, false, 0.0, "Annual".to_string(), "10% per annum".to_string()),
        StateRate::new("Arkansas".to_string(), 8.0, false, 0.0, "Annual".to_string(), "8% per annum".to_string()),
        StateRate::new("California".to_string(), 10.0, false, 0.0, "Annual".to_string(), "10% per annum".to_string()),
        StateRate::new("Colorado".to_string(), 8.0, false, 0.0, "Annual".to_string(), "8% per annum compounded annually".to_string()),
        StateRate::new("Connecticut".to_string(), 8.0, false, 0.0, "Annual".to_string(), "8% per annum".to_string()),
        StateRate::new("Delaware".to_string(), 0.0, true, 5.0, "Annual".to_string(), "Federal rate + 5%".to_string()),
        StateRate::new("District of Columbia".to_string(), 6.0, false, 0.0, "Annual".to_string(), "6% per annum".to_string()),
        StateRate::new("Florida".to_string(), 4.75, false, 0.0, "Annual".to_string(), "4.75% per annum as of 2024".to_string()),
        StateRate::new("Georgia".to_string(), 7.0, false, 0.0, "Annual".to_string(), "7% per annum".to_string()),
        StateRate::new("Hawaii".to_string(), 10.0, false, 0.0, "Annual".to_string(), "10% per annum".to_string()),
        StateRate::new("Idaho".to_string(), 12.0, false, 0.0, "Annual".to_string(), "12% per annum".to_string()),
        StateRate::new("Illinois".to_string(), 9.0, false, 0.0, "Annual".to_string(), "9% per annum".to_string()),
        StateRate::new("Indiana".to_string(), 8.0, false, 0.0, "Annual".to_string(), "8% per annum".to_string()),
        StateRate::new("Iowa".to_string(), 5.0, true, 2.0, "Annual".to_string(), "Federal rate + 2%".to_string()),
        StateRate::new("Kansas".to_string(), 10.0, false, 0.0, "Annual".to_string(), "10% per annum".to_string()),
        StateRate::new("Kentucky".to_string(), 8.0, false, 0.0, "Annual".to_string(), "8% per annum".to_string()),
        StateRate::new("Louisiana".to_string(), 3.5, false, 0.0, "Annual".to_string(), "3.5% per annum".to_string()),
        StateRate::new("Maine".to_string(), 8.0, false, 0.0, "Annual".to_string(), "8% per annum".to_string()),
        StateRate::new("Maryland".to_string(), 10.0, false, 0.0, "Annual".to_string(), "10% per annum".to_string()),
        StateRate::new("Massachusetts".to_string(), 12.0, false, 0.0, "Annual".to_string(), "12% per annum".to_string()),
        StateRate::new("Michigan".to_string(), 5.25, false, 0.0, "Semi-Annual".to_string(), "5.25% per annum".to_string()),
        StateRate::new("Minnesota".to_string(), 10.0, false, 0.0, "Annual".to_string(), "10% per annum".to_string()),
        StateRate::new("Mississippi".to_string(), 8.0, false, 0.0, "Annual".to_string(), "8% per annum".to_string()),
        StateRate::new("Missouri".to_string(), 9.0, false, 0.0, "Annual".to_string(), "9% per annum".to_string()),
        StateRate::new("Montana".to_string(), 10.0, false, 0.0, "Annual".to_string(), "10% per annum".to_string()),
        StateRate::new("Nebraska".to_string(), 12.0, false, 0.0, "Annual".to_string(), "12% per annum".to_string()),
        StateRate::new("Nevada".to_string(), 5.25, false, 0.0, "Annual".to_string(), "Prime rate + 2%".to_string()),
        StateRate::new("New Hampshire".to_string(), 7.0, false, 0.0, "Annual".to_string(), "7% per annum".to_string()),
        StateRate::new("New Jersey".to_string(), 8.75, false, 0.0, "Annual".to_string(), "8.75% per annum".to_string()),
        StateRate::new("New Mexico".to_string(), 15.0, false, 0.0, "Annual".to_string(), "15% per annum".to_string()),
        StateRate::new("New York".to_string(), 9.0, false, 0.0, "Annual".to_string(), "9% per annum".to_string()),
        StateRate::new("North Carolina".to_string(), 8.0, false, 0.0, "Annual".to_string(), "8% per annum".to_string()),
        StateRate::new("North Dakota".to_string(), 12.0, false, 0.0, "Annual".to_string(), "12% per annum".to_string()),
        StateRate::new("Ohio".to_string(), 3.0, true, 0.0, "Annual".to_string(), "Federal rate + 3%".to_string()),
        StateRate::new("Oklahoma".to_string(), 4.5, true, 4.0, "Annual".to_string(), "Prime rate + 4%".to_string()),
        StateRate::new("Oregon".to_string(), 9.0, false, 0.0, "Annual".to_string(), "9% per annum".to_string()),
        StateRate::new("Pennsylvania".to_string(), 6.0, false, 0.0, "Annual".to_string(), "6% per annum".to_string()),
        StateRate::new("Rhode Island".to_string(), 12.0, false, 0.0, "Annual".to_string(), "12% per annum".to_string()),
        StateRate::new("South Carolina".to_string(), 8.75, false, 0.0, "Annual".to_string(), "8.75% per annum".to_string()),
        StateRate::new("South Dakota".to_string(), 10.0, false, 0.0, "Annual".to_string(), "10% per annum".to_string()),
        StateRate::new("Tennessee".to_string(), 10.0, false, 0.0, "Annual".to_string(), "10% per annum".to_string()),
        StateRate::new("Texas".to_string(), 5.0, false, 0.0, "Annual".to_string(), "5% per annum if parties agree, 18% otherwise".to_string()),
        StateRate::new("Utah".to_string(), 2.0, true, 0.0, "Annual".to_string(), "Federal rate + 2%".to_string()),
        StateRate::new("Vermont".to_string(), 12.0, false, 0.0, "Annual".to_string(), "12% per annum".to_string()),
        StateRate::new("Virginia".to_string(), 6.0, false, 0.0, "Annual".to_string(), "6% per annum".to_string()),
        StateRate::new("Washington".to_string(), 12.0, false, 0.0, "Annual".to_string(), "12% per annum or contract rate".to_string()),
        StateRate::new("West Virginia".to_string(), 7.0, false, 0.0, "Annual".to_string(), "7% per annum".to_string()),
        StateRate::new("Wisconsin".to_string(), 5.0, false, 0.0, "Annual".to_string(), "5% per annum".to_string()),
        StateRate::new("Wyoming".to_string(), 10.0, false, 0.0, "Annual".to_string(), "10% per annum".to_string()),
    ];

    for state in initial_states {
        insert_state_rate(conn, &state)?;
    }

    Ok(())
}

pub fn get_connection(app: &AppHandle) -> Result<Connection> {
    let db_path = get_db_path(app);
    Connection::open(db_path)
}

pub fn get_state_rate(app: &AppHandle, state: &str) -> Result<Option<StateRate>> {
    let conn = get_connection(app)?;
    let mut stmt = conn.prepare(
        "SELECT id, state, rate, is_variable, plus_percentage, update_frequency, last_update, notes
         FROM state_rates WHERE state = ?1"
    )?;
    
    let state_rate = stmt.query_row(params![state], |row| {
        Ok(StateRate {
            id: row.get(0)?,
            state: row.get(1)?,
            rate: row.get(2)?,
            is_variable: row.get(3)?,
            plus_percentage: row.get(4)?,
            update_frequency: row.get(5)?,
            last_update: row.get(6)?,
            notes: row.get(7)?,
        })
    }).optional()?;

    Ok(state_rate)
}

pub fn get_all_states(app: &AppHandle) -> Result<Vec<StateRate>> {
    let conn = get_connection(app)?;
    let mut stmt = conn.prepare(
        "SELECT id, state, rate, is_variable, plus_percentage, update_frequency, last_update, notes
         FROM state_rates ORDER BY state"
    )?;
    
    let state_rates = stmt.query_map([], |row| {
        Ok(StateRate {
            id: row.get(0)?,
            state: row.get(1)?,
            rate: row.get(2)?,
            is_variable: row.get(3)?,
            plus_percentage: row.get(4)?,
            update_frequency: row.get(5)?,
            last_update: row.get(6)?,
            notes: row.get(7)?,
        })
    })?;

    state_rates.collect()
}

pub fn update_state_rate(app: &AppHandle, state_rate: &StateRate) -> Result<()> {
    let conn = get_connection(app)?;
    conn.execute(
        "UPDATE state_rates SET rate = ?1, is_variable = ?2, plus_percentage = ?3,
         update_frequency = ?4, last_update = ?5, notes = ?6 WHERE id = ?7",
        params![
            state_rate.rate,
            state_rate.is_variable,
            state_rate.plus_percentage,
            state_rate.update_frequency,
            state_rate.last_update,
            state_rate.notes,
            state_rate.id
        ],
    )?;
    Ok(())
}

pub fn insert_state_rate(conn: &Connection, state_rate: &StateRate) -> Result<()> {
    conn.execute(
        "INSERT INTO state_rates (state, rate, is_variable, plus_percentage, update_frequency, last_update, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            state_rate.state,
            state_rate.rate,
            state_rate.is_variable,
            state_rate.plus_percentage,
            state_rate.update_frequency,
            state_rate.last_update,
            state_rate.notes
        ],
    )?;
    Ok(())
}

pub fn delete_state_rate(app: &AppHandle, id: i32) -> Result<()> {
    let conn = get_connection(app)?;
    conn.execute("DELETE FROM state_rates WHERE id = ?1", params![id])?;
    Ok(())
}
