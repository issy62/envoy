use sonic_rs::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// `TwilioSettings` holds the configuration settings for the Twilio service.
///
/// # Fields
///
/// * `sid`: The Twilio account SID
/// * `auth_token`: The Twilio auth token
/// * `account_number`: The Twilio account phone number
#[derive(Serialize, Deserialize)]
pub struct TwilioSettings {
    /// The Twilio account SID
    pub sid: String,
    /// The Twilio auth token
    pub auth_token: String,
    /// The Twilio account phone number
    pub account_number: String,
}

impl TwilioSettings {
    /// Creates a new TwilioSettings
    ///
    /// # Arguments
    ///
    /// * `sid`: The Twilio account SID
    /// * `auth_token`: The Twilio auth token
    /// * `account_number`: The Twilio account phone number
    #[inline]
    pub fn new(sid: &str, auth_token: &str, account_number: &str) -> Self {
        Self {
            sid: sid.to_string(),
            auth_token: auth_token.to_string(),
            account_number: account_number.to_string(),
        }
    }
}

impl Default for TwilioSettings {
    fn default() -> Self {
        Self {
            sid: "".to_string(),
            auth_token: "".to_string(),
            account_number: "".to_string(),
        }
    }
}

/// `SentrySettings` holds the configuration settings for the Sentry service.
///
/// # Fields
///
/// * `dsn`: The Sentry DSN (Data Source Name)
/// * `debug`: Toggle debug mode
#[derive(Serialize, Deserialize)]
pub struct SentrySettings {
    /// The Sentry DSN (Data Source Name)
    pub dsn: String,
    /// Toggle debug mode
    pub debug: bool,
}

impl SentrySettings {
    /// Creates a new SentrySettings
    ///
    /// # Arguments
    /// * `dsn`: The Sentry DSN (Data Source Name)
    /// * `debug`: Toggle debug mode
    #[inline]
    pub fn new(dsn: &str, debug: bool) -> Self {
        Self {
            dsn: dsn.to_string(),
            debug,
        }
    }
}

impl Default for SentrySettings {
    fn default() -> Self {
        Self {
            dsn: "".to_string(),
            debug: false,
        }
    }
}

/// `TursoSettings` holds the configuration settings for the Turso service.
///
/// # Fields
///
/// * `database`: The Turso database name
/// * `auth_token`: The Turso auth token
#[derive(Serialize, Deserialize)]
pub struct TursoSettings {
    /// The Turso database name
    pub database: String,
    /// The Turso auth token
    pub auth_token: String,
}

impl TursoSettings {
    /// Creates a new TursoSettings
    ///
    /// # Arguments
    ///
    /// * `database`: The Turso database name
    /// * `auth_token`: The Turso auth token
    #[inline]
    pub fn new(database: &str, auth_token: &str) -> Self {
        Self {
            database: database.to_string(),
            auth_token: auth_token.to_string(),
        }
    }
}

impl Default for TursoSettings {
    fn default() -> Self {
        Self {
            database: "".to_string(),
            auth_token: "".to_string(),
        }
    }
}

/// `ConfigContext` holds the configuration settings for various services
/// used in the application.
///
/// # Fields:
///
/// * `sentry`: An instance of `SentrySettings` that holds the configuration for the Sentry service.
/// * `twilio`: An instance of `TwilioSettings` that holds the configuration for the Twilio service.
/// * `turso`: An instance of `TursoSettings` that holds the configuration for the Turso service.
#[derive(Serialize, Deserialize, Default)]
pub struct ConfigContext {
    /// An instance of `SentrySettings` that holds the configuration for the Sentry service.
    pub sentry: SentrySettings,
    /// An instance of `TwilioSettings` that holds the configuration for the Twilio service.
    pub twilio: TwilioSettings,
    /// An instance of `TursoSettings` that holds the configuration for the Turso service.
    pub turso: TursoSettings,
}

impl ConfigContext {
    /// Creates a new ConfigContext
    ///
    /// # Arguments
    ///
    /// * `sentry`: An instance of `SentrySettings` that holds the configuration for the Sentry service.
    /// * `twilio`: An instance of `TwilioSettings` that holds the configuration for the Twilio service.
    /// * `turso`: An instance of `TursoSettings` that holds the configuration for the Turso service.
    #[inline]
    pub fn new(sentry: SentrySettings, twilio: TwilioSettings, turso: TursoSettings) -> Self {
        Self {
            sentry,
            twilio,
            turso,
        }
    }

    /// Loads the config from the given path.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the path to the config file.
    ///
    /// # Returns
    ///
    /// * `Ok(()) - The config was loaded successfully.
    /// * `Err(std::io::Error)` - An error if the config could not be loaded.
    pub fn load_config(&mut self, path: &str) -> Result<(), std::io::Error> {
        let config_path = Path::new(path);

        let mut config_file = match File::open(config_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("error: {}", e);
                return Err(e);
            }
        };

        let mut file_content = String::new();
        match config_file.read_to_string(&mut file_content) {
            Ok(_) => {
                *self = sonic_rs::from_str(&file_content).unwrap_or_default();
            }
            Err(e) => {
                eprintln!("error: {}", e);
                return Err(e);
            }
        };

        Ok(())
    }
}

