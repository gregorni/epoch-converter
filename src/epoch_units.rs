use crate::timestamp::Timestamp;
use crate::units::units;

/// Convert epoch time to units of time on the Gregorian calendar.
pub fn epoch_units(seconds: u64) -> Timestamp {
    let seconds_str = format!("{}", seconds);
    let epoch_seconds = match seconds_str.len() {
        // This code will break in Gregorian calendar year 2286.
        12 => {
            let unrounded = (seconds / 100) as f64;
            println!(
                "{} seems to be in milliseconds, and will be cut down to {} seconds.",
                seconds,
                unrounded.floor()
            );
            unrounded.floor() as u64
        }
        13 => {
            let unrounded = (seconds / 1000) as f64;
            println!(
                "{} seems to be in milliseconds, and will be cut down to {} seconds.",
                seconds,
                unrounded.floor()
            );
            unrounded.floor() as u64
        }
        14 => {
            let unrounded = (seconds / 10000) as f64;
            println!(
                "{} seems to be in milliseconds, and will be cut down to {} seconds.",
                seconds,
                unrounded.floor()
            );
            unrounded.floor() as u64
        }
        15 => {
            let unrounded = (seconds / 100000) as f64;
            println!(
                "{} seems to be in microseconds, and will be cut down to {} seconds.",
                seconds,
                unrounded.floor()
            );
            unrounded.floor() as u64
        }
        16 => {
            let unrounded = (seconds / 1000000) as f64;
            println!(
                "{} seems to be in microseconds, and will be cut down to {} seconds.",
                seconds,
                unrounded.floor()
            );
            unrounded.floor() as u64
        }
        17 => {
            let unrounded = (seconds / 10000000) as f64;
            println!(
                "{} seems to be in nanoseconds, and will be cut down to {} seconds.",
                seconds,
                unrounded.floor()
            );
            unrounded.floor() as u64
        }
        18 => {
            let unrounded = (seconds / 100000000) as f64;
            println!(
                "{} seems to be in nanoseconds, and will be cut down to {} seconds.",
                seconds,
                unrounded.floor()
            );
            unrounded.floor() as u64
        }
        19 => {
            let unrounded = (seconds / 1000000000) as f64;
            println!(
                "{} seems to be in nanoseconds, and will be cut down to {} seconds.",
                seconds,
                unrounded.floor()
            );
            unrounded.floor() as u64
        }
        20 => {
            let unrounded = (seconds / 1000000000) as f64;
            println!(
                "{} seems to be in nanoseconds, and will be cut down to {} seconds.",
                seconds,
                unrounded.floor()
            );
            unrounded.floor() as u64
        }
        _ => seconds,
    };

    let time = units(epoch_seconds);

    Timestamp {
        seconds: time.seconds + 62135769600,
        minutes: time.minutes + 1035596160,
        hours: time.hours + 17259936,
        days: time.days + 719164,
        weeks: time.weeks + 102737,
        months: time.months + 23628,
        years: time.years + 1970,
    }
}
