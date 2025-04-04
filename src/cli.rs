use anyhow::{anyhow, Result};
use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Time to convert (timestamp, 'now', or datetime string)
    #[arg(value_name = "TIME")]
    pub time: String,

    /// Timezone(s) to convert to (can be specified multiple times)
    #[arg(short = 'z', long = "timezone")]
    pub timezones: Vec<String>,
}

pub fn get_timezones(args: &Args) -> Result<Vec<String>> {
    // If timezones are specified via command line, use only those
    if !args.timezones.is_empty() {
        return Ok(args.timezones.clone());
    }

    // Otherwise, try environment variable
    if let Ok(tz) = env::var("GTIME_TIMEZONES") {
        return Ok(tz.split(',').map(|s| s.trim().to_string()).collect());
    }

    // Finally, try config file
    if let Ok(config) = crate::config::load_config() {
        if !config.timezones.is_empty() {
            return Ok(config.timezones);
        }
    }

    Err(anyhow!("No timezone specified. Use -z or --timezone to specify one or more timezones, set GTIME_TIMEZONES environment variable, or configure timezones in ~/.config/gtime/config.toml"))
} 