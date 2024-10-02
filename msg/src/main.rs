use clap::Parser;
use libsql::errors::Error as SqlError;
use libsql::Connection as SqlConnection;
use msg::config::*;
use msg::logger::*;
use std::time::Duration;

pub fn init_sentry(config: &SentrySettings) -> sentry::ClientInitGuard {
    let guard = sentry::init((
        config.dsn.to_string(),
        sentry::ClientOptions {
            attach_stacktrace: true,
            debug: config.debug,
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    if !guard.is_enabled() {
        eprintln!("Sentry is not enabled.");
    }

    guard
}

pub async fn init_turso(config: &TursoSettings) -> Result<SqlConnection, SqlError> {
    let connection =
        libsql::Builder::new_remote(config.database.to_string(), config.auth_token.to_string())
            .build()
            .await;

    match connection {
        Err(e) => return Err(e),
        Ok(r) => match r.connect() {
            Err(e) => return Err(e),
            Ok(r) => return Ok(r),
        },
    };
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Flags {
    #[arg(short, long, default_value = "config.json")]
    config_path: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let flags = Flags::parse();

    let config_path = flags.config_path.as_deref().unwrap_or_default();

    let mut conf = ConfigContext::default();
    conf.load_config(config_path)?;

    let guard = init_sentry(&conf.sentry);

    let tr = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Unable to create Tokio runtime");

    tr.block_on(async {
        let db = init_turso(&conf.turso).await;

        match db {
            Err(e) => {
                eprintln!("Error: {}", e);
                sentry::capture_error(&e);
            }
            Ok(r) => {
                let log_ctx = LoggerContext::new("TEST", "TEST", "TEST", "TEST", "TEST", "TEST");
                let res = log_to_turso(&r, log_ctx).await;
                match res {
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        sentry::capture_error(&e);
                    }
                    Ok(r) => println!("{} log entry inserted.", r),
                }
            }
        }
    });

    guard.flush(Some(Duration::from_secs(2)));

    Ok(())
}

