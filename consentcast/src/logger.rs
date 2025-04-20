use clap::ArgMatches;
use serde::{Serialize, Deserialize};
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc;

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
    let consent_given = matches.get_one::<String>("consent").unwrap().parse::<bool>().unwrap();

    let pii_fields: Vec<String> = fields
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

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
        .open("consent_logs.json")
        .expect("Failed to open log file");

    writeln!(file, "{}", json).expect("Failed to write log");

    println!("Consent recorded successfully.");
}
