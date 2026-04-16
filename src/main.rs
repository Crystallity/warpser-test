// Webserver test with warp

mod input;
mod config;

use config::Config;
use clap::Parser;
use crate::input::ConfigNotFoundError;

/// Simple webserver capable of exposing a frontend website and/or providing file hosting
#[derive(Parser)]
#[command(name = "Warpser", version, about, long_about = None)]
struct Args {
    /// Load the specified config file
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[arg(short, long, default_value = "/usr/share/warpser/config.toml")]
    config: String,

    /// Load the specified config file
    #[cfg(target_os = "windows")]
    #[arg(short, long, default_value = "%LocalAppData%/warpser/config.toml")]
    config: String,

    /// Expose this directory as a frontend website (override config)
    #[arg(short, long)]
    web_dir: Option<String>,

    /// Allow configured users to host files in this directory (override config)
    #[arg(short, long)]
    file_dir: Option<String>,

    /// Write logs to this directory
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[arg(short, long, default_value = "/var/lib/warpser/logs")]
    log_dir: String,

    /// Write logs to this directory
    #[cfg(target_os = "windows")]
    #[arg(short, long, default_value = "%LocalAppData%/warpser/logs")]
    log_dir: String
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let args = Args::parse();

    // Load config file
    let mut config: Config = match input::config(&args.config) {
        Ok(c) => {c}
        Err(ConfigNotFoundError) => {
            if args.file_dir == None && args.web_dir == None {
                println!("No config file or options available, quitting");
                std::process::exit(1);
            } else {
                Config { files: crate::config::Files {web_dir: None, file_dir: None}, users: crate::config::Users {} }
            }
        }
    };

    if args.file_dir == None && args.web_dir == None && config.files.web_dir == None && config.files.web_dir == None {
        println!("No options available, quitting");
        return;
    }

    if args.web_dir != None {
        config.files.web_dir = args.web_dir;
    }
    if args.file_dir != None {
        config.files.file_dir = args.file_dir;
    }

    // Unix-like systems require root priviledges to bind to port 80
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    if sudo::check() == sudo::RunningAs::User {
        if input::yes_no("You need to be root to run the server!\nElevate to root?") {
            println!();
            
            sudo::escalate_if_needed().expect("Failed to run as root. Try running with sudo.");
        }
    }

    // Serve site directory
    let site = warp::fs::dir(config.files.web_dir.unwrap());
    
    // Start server
    println!("Hosting server on port 80");
    warp::serve(site).run(([0, 0, 0, 0], 80)).await;

}
