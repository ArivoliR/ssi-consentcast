//use clap::{Arg, ArgMatches, Command};
use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("consentcast")
        .about("Consent Logging CLI for Fintech Compliance")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("record")
                .about("Record a data sharing consent event")
                .arg(Arg::new("user").long("user").required(true))
                .arg(Arg::new("recipient").long("recipient").required(true))
                .arg(Arg::new("fields").long("fields").required(true))
                .arg(Arg::new("purpose").long("purpose").required(true))
                .arg(Arg::new("consent").long("consent").required(true)),
        )
        .subcommand(
            Command::new("audit")
                .about("Audit consent logs")
                .arg(Arg::new("user").long("user").required(false))
                .arg(Arg::new("recipient").long("recipient").required(false)),
        )
        .subcommand(
            Command::new("hash")
                .about("Generate hash of logs")
                .arg(Arg::new("file").long("file").required(true)),
        )
        .subcommand(
            Command::new("sign")
                .about("Sign consent log file with private key")
                .arg(Arg::new("file").long("file").required(true)),
        )
        .subcommand(Command::new("keygen").about("Generate Ed25519 keypair"))
        .subcommand(
            Command::new("verify")
                .about("Verify digital signature of consent log file")
                .arg(Arg::new("file").long("file").required(true)),
        )
}
