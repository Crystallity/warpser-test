use std::io::Write;
use log::{info, warn, trace};

use crate::config::Config;

// Error type used in config()
type Result<T> = std::result::Result<T, ConfigNotFoundError>;
#[derive(Debug)]
pub(crate) struct ConfigNotFoundError;

// Asks a yes or no question and returns a boolean corresponding to the user's answer.
// ask: &str - The question to ask the user
// return: bool - The user's response. True = 'yes', False = 'no'
pub(crate) fn yes_no(ask: &str) -> bool {
    trace!("Asking the user a y/n question...");

    // Ask the question
    print!("{} ", ask);

    // Counter for multiple attempts
    let mut attempt = 1;

    // Attempt 3 times or until successful
    loop {
        let mut input = String::new();

        // Flush output to read user input
        match std::io::stdout().flush() {
            Ok(..) => {}
            Err(e) => {
                warn!("Could not flush stdout (attempt {}, trying {} more times): {}", attempt, 3 - attempt, e);
                attempt += 1;
                if attempt > 3 {
                    panic!("Failed to flush output! Run with RUST_LOG=trace for more info.");
                }
                continue;
            }
        }

        // Read input and return answer (recurse if incorrect)
        match std::io::stdin().read_line(&mut input) {
            Ok(b) => {
                info!("Read {} bytes: {}\\n", b, input[0..input.len() - 1].to_string());
                match input.to_uppercase().as_str() {
                    "Y\n" | "YES\n" => {return true;}
                    "N\n" | "NO\n" => {return false;}
                    _ => {
                        println!("\nPlease enter 'y', 'yes', 'n', or 'no'\n");
                        return yes_no(ask);
                    }
                }
            }
            Err(e) => {
                warn!("Could not read stdin (attempt {}, trying {} more times): {}", attempt, 3 - attempt, e);
                attempt += 1;
                if attempt > 3 {
                    panic!("Failed to read input! Run with RUST_LOG=trace for more info.");
                }
                continue;
            }
        }
    }
}

pub(crate) fn config(path: &String) -> Result<crate::Config> {
    // Counter for multiple attempts
    let mut attempt = 1;

    // Loop until success or 3 errors
    loop {
        let serial: String;
        match std::fs::read_to_string(&path) {
            Ok(s) => {serial = s;}
            Err(e) => {
                warn!("Could not read config file (attempt {}, trying {} more times): {}", attempt, 3 - attempt, e);
                attempt += 1;
                if attempt > 3 {
                    break;
                }
                continue;
            }
        }

        match toml::from_str::<Config>(serial.as_str()) {
            Ok(c) => {return Ok(c);}
            Err(e) => {
                warn!("Could not deserialize config file (attempt {}, trying {} more times): {}", attempt, 3 - attempt, e);
                attempt += 1;
                if attempt > 3 {
                    break;
                }
                continue;
            }
        }
    }

    return Err(ConfigNotFoundError);
}


