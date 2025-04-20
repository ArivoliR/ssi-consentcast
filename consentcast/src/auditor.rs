use crate::logger::ConsentLog;
use clap::ArgMatches;
use colored::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn handle_audit(matches: &ArgMatches) {
    let user_filter = matches.get_one::<String>("user");
    let recipient_filter = matches.get_one::<String>("recipient");

    let file = match File::open("consent_logs.json") {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Consent log file not found.");
            return;
        }
    };

    let reader = BufReader::new(file);
    let mut found = false;

    for line in reader.lines() {
        if let Ok(json) = line {
            if let Ok(log) = serde_json::from_str::<ConsentLog>(&json) {
                let user_match = user_filter.map_or(true, |u| u == &log.user_id);
                let recipient_match = recipient_filter.map_or(true, |r| r == &log.recipient);

                if user_match && recipient_match {
                    found = true;
                    println!("{}", "──────────────".bright_black());
                    println!("User: {}", log.user_id.cyan());
                    println!("Fields: {}", log.pii_fields.join(", "));
                    println!("Recipient: {}", log.recipient.yellow());
                    println!("Purpose: {}", log.purpose.green());
                    println!("Timestamp: {}", log.timestamp);
                    println!(
                        "Consent: {}",
                        if log.consent_given {
                            "Yes".bright_green()
                        } else {
                            "No".red()
                        }
                    );
                }
            }
        }
    }

    if !found {
        println!("No matching logs found");
    }
}
