use clap::Parser;
use finch::twilio::e164::*;
use msg::config::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Flags {
    #[arg(short, long, default_value = "config.json")]
    config: Option<String>,
}

fn main() {
    let flags = Flags::parse();

    let config_path = flags.config.as_deref().unwrap_or_default();

    let mut conf = ConfigContext::default();
    match conf.load_config(config_path) {
        Ok(_) => {
            println!("Twilio SID: {}", conf.twilio.sid);
            println!("Sentry DSN: {}", conf.sentry.dsn);
            println!("Turso Database: {}", conf.turso.database);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    match normalize_number("1234567890123") {
        Ok(r) => println!("Sanitized Number: {}", r),
        Err(e) => eprintln!("Error: {}", e),
    }
}

