use crate::timestamp::Timestamp;

/// Convert seconds into various units of time.
pub fn units(seconds: u64) -> Timestamp {
    let minutes_unrounded: f64 = (seconds / 60) as f64;
    let minutes = minutes_unrounded.floor() as u64;

    let hours_unrounded: f64 = (seconds / 3600) as f64;
    let hours = hours_unrounded.floor() as u64;

    let days_unrounded: f64 = (seconds / 86400) as f64;
    let days = days_unrounded.floor() as u64;

    let weeks_unrounded: f64 = (seconds / 604800) as f64;
    let weeks = weeks_unrounded.floor() as u64;

    let months_unrounded: f64 = (seconds / 2629743) as f64; // 30.44 days
    let months = months_unrounded.floor() as u64;

    let years_unrounded: f64 = (seconds / 31556926) as f64; // 365.24 days
    let years = years_unrounded.floor() as u64;

    Timestamp {
        seconds,
        minutes,
        hours,
        days,
        weeks,
        months,
        years,
    }
}