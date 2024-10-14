use finch::twilio::e164::*;

fn main() {
    match normalize_number("1234567890123") {
        Ok(r) => println!("Sanitized Number: {}", r),
        Err(e) => eprintln!("Error: {}", e),
    }
}

