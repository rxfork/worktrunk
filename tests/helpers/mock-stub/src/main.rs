//! Config-driven mock executable for integration tests.
//!
//! Reads a JSON config file to determine responses. When invoked as `gh`,
//! looks for `gh.json` in the same directory and responds based on config.
//!
//! Config format:
//! ```json
//! {
//!   "version": "gh version 2.0.0 (mock)",
//!   "commands": {
//!     "auth": { "exit_code": 0 },
//!     "pr": { "file": "pr_data.json" },
//!     "run": { "output": "[{\"status\": \"completed\"}]" }
//!   }
//! }
//! ```
//!
//! Command matching:
//! - `gh --version` → outputs version string
//! - `gh auth ...` → matches "auth" command
//! - `gh pr list ...` → matches "pr" command
//!
//! Response types:
//! - `file`: read and output contents of specified file (relative to config dir)
//! - `output`: output literal string
//! - `exit_code`: exit with specified code (default 0)

use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

#[derive(Debug, Deserialize)]
struct Config {
    version: Option<String>,
    #[serde(default)]
    commands: HashMap<String, CommandResponse>,
}

#[derive(Debug, Deserialize)]
struct CommandResponse {
    file: Option<String>,
    output: Option<String>,
    #[serde(default)]
    exit_code: i32,
}

fn main() {
    let exe_path = env::current_exe().expect("failed to get executable path");
    let exe_dir = exe_path
        .parent()
        .expect("mock: executable has no parent directory");

    let cmd_name = exe_path
        .file_stem()
        .expect("mock: executable has no file stem")
        .to_string_lossy();

    let config_path = exe_dir.join(format!("{}.json", cmd_name));

    let content = fs::read_to_string(&config_path).unwrap_or_else(|e| {
        eprintln!("mock: failed to read {}: {}", config_path.display(), e);
        exit(1);
    });

    let config: Config = serde_json::from_str(&content).unwrap_or_else(|e| {
        eprintln!("mock: failed to parse {}: {}", config_path.display(), e);
        exit(1);
    });

    let args: Vec<String> = env::args().skip(1).collect();

    // Handle --version flag
    if args.first().map(|s| s.as_str()) == Some("--version")
        && let Some(version) = &config.version
    {
        println!("{}", version);
        exit(0);
    }

    // Match first argument against commands, fall back to _default
    let default_response = CommandResponse {
        file: None,
        output: None,
        exit_code: 1,
    };
    let response = args
        .first()
        .and_then(|cmd| config.commands.get(cmd))
        .or_else(|| config.commands.get("_default"))
        .unwrap_or(&default_response);

    if let Some(file) = &response.file {
        let file_path = exe_dir.join(file);
        match fs::read_to_string(&file_path) {
            Ok(contents) => {
                print!("{}", contents);
                io::stdout().flush().unwrap();
            }
            Err(e) => {
                eprintln!("mock: failed to read {}: {}", file_path.display(), e);
                exit(1);
            }
        }
    } else if let Some(output) = &response.output {
        print!("{}", output);
        io::stdout().flush().unwrap();
    }

    exit(response.exit_code);
}
