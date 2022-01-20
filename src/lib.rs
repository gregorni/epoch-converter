mod timestamp;
use timestamp::Timestamp;

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

#[cfg(test)]
mod tests {
    use crate::{epoch_units, units};
    // Test units()
    #[test]
    fn test_seconds() {
        assert_eq!(1, units(1).seconds);
        assert_eq!(10, units(10).seconds);
    }

    #[test]
    fn test_minutes() {
        assert_eq!(1, units(60).minutes);
        assert_eq!(10, units(600).minutes);
    }

    #[test]
    fn test_hours() {
        assert_eq!(1, units(3600).hours);
        assert_eq!(10, units(36000).hours);
    }

    #[test]
    fn test_days() {
        assert_eq!(1, units(86400).days);
        assert_eq!(10, units(864000).days);
    }

    #[test]
    fn test_weeks() {
        assert_eq!(1, units(604800).weeks);
        assert_eq!(10, units(6048000).weeks);
    }

    #[test]
    fn test_months() {
        assert_eq!(1, units(2629743).months);
        assert_eq!(10, units(26297430).months);
    }

    #[test]
    fn test_years() {
        assert_eq!(1, units(31556926).years);
        assert_eq!(10, units(315569260).years);
    }

    // Test epoch_units(), using epoch time 1000000000, which is Sunday, September 9, 2001 1:46:40 AM UTC±00:00.
    #[test]
    fn test_epoch_seconds() {
        assert_eq!(63135769600, epoch_units(1000000000).seconds)
    }

    #[test]
    fn test_epoch_minutes() {
        assert_eq!(1052262826, epoch_units(1000000000).minutes)
    }

    #[test]
    fn test_epoch_hours() {
        assert_eq!(17537713, epoch_units(1000000000).hours)
    }

    #[test]
    fn test_epoch_days() {
        assert_eq!(730738, epoch_units(1000000000).days)
    }

    #[test]
    fn test_epoch_weeks() {
        assert_eq!(104390, epoch_units(1000000000).weeks)
    }

    #[test]
    fn test_epoch_months() {
        assert_eq!(24008, epoch_units(1000000000).months)
    }
    #[test]
    fn test_epoch_years() {
        assert_eq!(2001, epoch_units(1000000000).years)
    }

    // Test epoch_units()'s check and convert
    #[test]
    fn milliseconds_convert() {
        assert_eq!(63135769600, epoch_units(1000000000000).seconds)
    }

    #[test]
    fn microseconds_convert() {
        assert_eq!(63135769600, epoch_units(1000000000000000).seconds)
    }

    #[test]
    fn nanoseconds_convert() {
        assert_eq!(63135769600, epoch_units(1000000000000000000).seconds)
    }

    #[test]
    fn maximum_time() {
        dbg!(epoch_units(18446744073709551615));
        assert_eq!(2554, epoch_units(18446744073709551615).years);
    }
}
