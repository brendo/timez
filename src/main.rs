mod cli;
mod config;
mod time;

use anyhow::Result;
use chrono::TimeZone;
use chrono_tz::Tz;
use clap::Parser;
use std::str::FromStr;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    let timezones = cli::get_timezones(&args)?;

    let (input_str, time, format) = time::parse_time(&args.time)?;

    // Convert and display times for each timezone
    println!("Input time: {}\n", input_str);
    for tz_name in timezones {
        match Tz::from_str(&tz_name) {
            Ok(tz) => {
                let local_time = tz.from_utc_datetime(&time.naive_utc());
                let offset = tz.offset_from_utc_datetime(&time.naive_utc());
                println!(
                    "{}: {}",
                    tz_name,
                    time::format_timezone_output(&tz_name, local_time, offset, format)
                );
            }
            Err(e) => eprintln!("Error: Invalid timezone '{}': {}", tz_name, e),
        }
    }

    Ok(())
}
