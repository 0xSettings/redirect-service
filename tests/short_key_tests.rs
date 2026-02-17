use redirect_service::model::short_key::ShortKey;
use redirect_service::model::errors::DomainError;

// valid key - 6 chars
#[test]
fn valid_short_key_is_accepted() {
    let result = ShortKey::new("aB3kR9".to_string());
    assert!(result.is_ok());
}

// valid key — if more than 6 char works
#[test]
fn short_key_longer_than_six_is_accepted() {
    let result = ShortKey::new("aB3kR9XyZ".to_string());
    assert!(result.is_ok());
}

// char is too short below 6 kindly reject 
#[test]
fn short_key_under_6_chars_is_rejected() {
    let result = ShortKey::new("abc".to_string());
    assert!(matches!(result, Err(DomainError::InvalidShortKey)));
}

// char withing 5 
#[test]
fn short_key_of_5_chars_is_rejected() {
    let result = ShortKey::new("abcde".to_string());
    assert!(matches!(result, Err(DomainError::InvalidShortKey)));
}

// empty string
#[test]
fn empty_short_key_is_rejected() {
    let result = ShortKey::new("".to_string());
    assert!(matches!(result, Err(DomainError::InvalidShortKey)));
}

// special characters kindly reject it 
#[test]
fn short_key_with_special_chars_is_rejected() {
    let result = ShortKey::new("aB3k!9".to_string());
    assert!(matches!(result, Err(DomainError::InvalidShortKey)));
}

// spaces involved - reject it 
#[test]
fn short_key_with_spaces_is_rejected() {
    let result = ShortKey::new("aB3k 9".to_string());
    assert!(matches!(result, Err(DomainError::InvalidShortKey)));
}
