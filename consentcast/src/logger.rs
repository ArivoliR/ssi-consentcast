use chrono::Utc;
use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConsentLog {
    pub user_id: String,
    pub pii_fields: Vec<String>,
    pub recipient: String,
    pub purpose: String,
    pub timestamp: String,
    pub consent_given: bool,
}

pub fn handle_record(matches: &ArgMatches) {
    let user_id = matches.get_one::<String>("user").unwrap().to_string();
    let recipient = matches.get_one::<String>("recipient").unwrap().to_string();
    let fields = matches.get_one::<String>("fields").unwrap();
    let purpose = matches.get_one::<String>("purpose").unwrap().to_string();
    let consent_given = matches
        .get_one::<String>("consent")
        .unwrap()
        .parse::<bool>()
        .unwrap();

    // Use --output or default to logs/consent_logs.json
    let output_path = matches
        .get_one::<String>("output")
        .map(|s| s.as_str())
        .unwrap_or("logs/consent_logs.json");

    // Create parent directory if needed
    if let Some(parent) = Path::new(output_path).parent() {
        if !parent.exists() {
            create_dir_all(parent).expect("Failed to create logs directory");
        }
    }

    let pii_fields: Vec<String> = fields.split(',').map(|s| s.trim().to_string()).collect();

    let log = ConsentLog {
        user_id,
        pii_fields,
        recipient,
        purpose,
        timestamp: Utc::now().to_rfc3339(),
        consent_given,
    };

    let json = serde_json::to_string(&log).unwrap();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(output_path)
        .expect("Failed to open log file");

    writeln!(file, "{}", json).expect("Failed to write log");

    println!("✅ Consent recorded at: {}", output_path);
}
