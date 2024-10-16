use finch::twilio::e164::*;
use msg::config::*;

fn main() {
    let mut conf = ConfigContext::default();

    match conf.load_config("/home/l0n353n7ry/Projects/envoy/msg/config.json") {
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

