use finch::twilio::e164::*;

#[test]
fn test_empty_number() {
    let input = "".to_string();
    let expected = E164ErrorType::EmptyNumber;
    let res = normalize_number(&input);

    match res {
        Ok(_) => assert!(false, "Expected an error, but got Ok"),
        Err(e) => {
            assert_eq!(e.error_type, expected)
        }
    }
}

#[test]
fn test_no_digit_input() {
    let input = "(abc) jty-aggg".to_string();
    let expected = E164ErrorType::EmptyAfterSanitization;
    let res = normalize_number(&input);

    match res {
        Ok(_) => assert!(false, "Expected an error, but got Ok"),
        Err(e) => {
            assert_eq!(e.error_type, expected)
        }
    }
}

#[test]
fn test_insufficient_number_length() {
    let input = "123456".to_string();
    let expected = E164ErrorType::InsufficientLength;
    let res = normalize_number(&input);

    match res {
        Ok(_) => assert!(false, "Expected an error, but got Ok"),
        Err(e) => {
            assert_eq!(e.error_type, expected)
        }
    }
}

#[test]
fn test_exceded_number_length() {
    let input = "5323456789012345".to_string();
    let expected = E164ErrorType::ExceededNumberLength;
    let res = normalize_number(&input);

    match res {
        Ok(_) => assert!(false, "Expected an error, but got Ok"),
        Err(e) => {
            assert_eq!(e.error_type, expected)
        }
    }
}

#[test]
fn successful_sanitization() {
    let input = "1 (813) 555-5555".to_string();
    let expected = "+18135555555".to_string();
    let res = normalize_number(&input);
    match res {
        Ok(r) => assert_eq!(r, expected),
        Err(_) => assert!(false, "Expected Ok, but got an error"),
    }
}

