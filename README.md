# epoch-converter

epoch-converter enables conversion between seconds and units of time as well as between an epoch timestamp and units of time.

## Important note

⚠️ epoch-converter _always_ rounds **down**.

## Usage

Add `epoch-converter = "0.1.0"` to your `Cargo.toml`

### units()

This function converts a number of seconds into various units of time:

- Seconds
- Minutes
- Hours
- Days
- Weeks
- Months (using 30.44 days per month)
- Years (using 365.24 days per year)

```rust
use epoch_converter::units;

fn main() {
    let seconds = 63115200;
    println!("{} years", units(seconds).years); // Print the number of years in 63115200 seconds.
    println!("{} months", units(seconds).months); // Print the number of months in 63115200 seconds.
    println!("{} days", units(seconds).days); // Print the number of days in 63115200 seconds.
}
```

### epoch_units()

This function converts an epoch timestamp into various units of time elapsed since Jan 1, 0001 on the Gregorian calendar:

- Seconds
- Minutes
- Hours
- Days
- Weeks
- Months (using 30.44 days per month)
- Years (using 365.24 days per year)

```rust
use epoch_converter::epoch_units;

fn main() {
    let epoch = 1278504000;
    println!("{} years", epoch_units(epoch).years); // Print the number of years in the epoch timestamp 1278504000.
    println!("{} months", epoch_units(epoch).months); // Print the number of months in the epoch timestamp 1278504000.
    println!("{} days", epoch_units(epoch).days); // Print the number of days in the epoch timestamp 1278504000.
}
```
