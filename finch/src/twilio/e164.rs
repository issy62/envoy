use std::cmp::PartialEq;

use std::error::Error;
use std::fmt;

/// Error types that can occur while performing a number normalization.
#[derive(Debug, PartialEq)]
pub enum E164ErrorType {
    /// The number input is empty
    EmptyNumber,
    /// The number input contained only non-numeric characters
    EmptyAfterSanitization,
    /// The number input is too short less than 8 digits this check is done after sanitization
    InsufficientLength,
    /// The number input is too long greater than 15 digits this check is done after sanitization
    ExceededNumberLength,
}

/// Represents an error that can occur while performing a number normalization.
#[derive(Debug)]
pub struct E164Error {
    pub error_type: E164ErrorType,
}

impl E164Error {
    /// Creates a new E164Error.
    pub fn new(error_type: E164ErrorType) -> Self {
        Self { error_type }
    }

    /// Returns the error message.
    pub fn message(&self) -> String {
        match self.error_type {
            E164ErrorType::EmptyNumber => "Empty number".to_string(),
            E164ErrorType::EmptyAfterSanitization => "No digits in input".to_string(),
            E164ErrorType::InsufficientLength => "Insufficient number length".to_string(),
            E164ErrorType::ExceededNumberLength => "Exceeded number length".to_string(),
        }
    }
}

impl fmt::Display for E164Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Error for E164Error {}

/// Normalizes a phone number to E.164 format.
///
///  # Arguments
///
/// * `number` - A string slice that holds the phone number to be normalized.
///
///  # Returns
///
///  * `Ok(String)` - The normalized phone number in E.164 format.
///  * `Err(E164Error)` - An error if the phone number is invalid.
pub fn normalize_number(number: &str) -> Result<String, E164Error> {
    if number.is_empty() {
        return Err(E164Error::new(E164ErrorType::EmptyNumber));
    }

    let mut cn: String = number.chars().filter(|c| c.is_ascii_digit()).collect();

    if cn.is_empty() {
        return Err(E164Error::new(E164ErrorType::EmptyAfterSanitization));
    }

    cn.insert(0, '+');

    if cn.len() > 16 {
        return Err(E164Error::new(E164ErrorType::ExceededNumberLength));
    }

    if cn.len() < 8 {
        return Err(E164Error::new(E164ErrorType::InsufficientLength));
    }

    Ok(cn)
}

