use clap::Parser;
use finch::twilio::e164::*;
use msg::config::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Flags {
    #[arg(short, long, default_value = "config.json")]
    config_path: Option<String>,
}

fn main() {
    let flags = Flags::parse();

    let config_path = flags.config_path.as_deref().unwrap_or_default();

    let mut conf = ConfigContext::default();
    match conf.load_config(config_path) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }

    let guard = sentry::init((
        conf.sentry.dsn.to_string(),
        sentry::ClientOptions {
            attach_stacktrace: true,
            debug: conf.sentry.debug,
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    if !guard.is_enabled() {
        eprintln!("Sentry is not enabled.");
    }

    match normalize_number("+1(656)210-7212") {
        Ok(r) => println!("Sanitized Number: {}", r),
        Err(e) => {
            sentry::capture_error(&e);
            eprintln!("Error: {}", e)
        }
    }

    guard.flush(Some(std::time::Duration::from_secs(4)));
}
