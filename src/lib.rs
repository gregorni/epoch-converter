pub mod epoch_units;
pub mod timestamp;
pub mod units;

#[cfg(test)]
mod tests {
    use crate::epoch_units::epoch_units;
    use crate::units::units;
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
