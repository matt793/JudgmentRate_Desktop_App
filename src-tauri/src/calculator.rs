use chrono::NaiveDate;

pub fn compute_interest(principal: f64, rate: f64, days: i64) -> f64 {
    // Simple interest formula: Interest = Principal × Rate × Time
    // Time is expressed as days / 365
    let interest = principal * rate * (days as f64 / 365.0);
    
    // Round to 2 decimal places
    (interest * 100.0).round() / 100.0
}

pub fn calculate_days_between(from_date: NaiveDate, to_date: NaiveDate) -> i64 {
    (to_date - from_date).num_days()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_interest() {
        // Test case: $10,000 at 5% for 365 days
        let principal = 10000.0;
        let rate = 0.05;
        let days = 365;
        
        let interest = compute_interest(principal, rate, days);
        assert_eq!(interest, 500.0);
    }
    
    #[test]
    fn test_compute_interest_partial_year() {
        // Test case: $10,000 at 10% for 180 days
        let principal = 10000.0;
        let rate = 0.10;
        let days = 180;
        
        let interest = compute_interest(principal, rate, days);
        // Should be approximately 493.15
        assert!((interest - 493.15).abs() < 0.01);
    }
    
    #[test]
    fn test_calculate_days_between() {
        let from_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let to_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        
        let days = calculate_days_between(from_date, to_date);
        assert_eq!(days, 365); // 2024 is a leap year, so 366 - 1
    }
}
