# timez

A command-line tool to convert timestamps between different timezones.

## Installation

```bash
cargo install --path .
```

## Usage

timez accepts timestamps in various formats and converts them to your configured timezones.

### Basic Usage

```bash
# Convert current time to configured timezones
timez "now"

# Convert a specific time
timez "2024-03-20 15:30:00"

# Convert an ISO8601 timestamp
timez "2024-03-20T15:30:00Z"

# Convert a Unix timestamp
timez "1710945000"
```

### Specifying Timezones

There are three ways to specify which timezones to convert to:

1. Command line arguments (highest priority):
```bash
timez "2024-03-20 15:30:00" -z "America/New_York" -z "Europe/London"
```

2. Environment variable:
```bash
export TIMEZ_TIMEZONES="America/New_York,Europe/London,Asia/Tokyo"
timez "2024-03-20 15:30:00"
```

3. Configuration file (lowest priority):
The default configuration file is `timezconfig.toml` in your OS's preferences directory:

- Linux `$HOME/.config`
- Windows `FOLDERID_RoamingAppData`
- macOS `$HOME/Library/Preferences`

```toml
timezones = [
    "America/New_York",
    "America/Los_Angeles",
    "Europe/London",
    "Asia/Tokyo",
    "Australia/Sydney"
]
```

## Supported Time Formats

- ISO8601 (e.g., "2024-03-20T15:30:00Z")
- Human-readable (e.g., "2024-03-20 15:30:00")
- Unix timestamp (e.g., "1710945000")
- "now" for current time

## Available Timezones

timez uses the IANA timezone database. You can find a list of available timezones at: https://en.wikipedia.org/wiki/List_of_tz_database_time_zones