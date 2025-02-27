use tavily_search::Error;

#[test]
fn test_error_from_string() {
    let error_message = "Test error message";
    let error = Error::from(error_message);

    match error {
        Error::General(msg) => assert_eq!(msg, error_message),
        _ => panic!("Expected Error::General"),
    }
}

#[test]
fn test_error_from_string_owned() {
    let error_message = String::from("Test error message");
    let error = Error::from(error_message.clone());

    match error {
        Error::General(msg) => assert_eq!(msg, error_message),
        _ => panic!("Expected Error::General"),
    }
}

#[test]
fn test_error_display() {
    let error = Error::Environment("Missing API key".to_string());
    let error_string = format!("{}", error);
    assert_eq!(error_string, "Environment error: Missing API key");
}

#[test]
fn test_error_debug() {
    let error = Error::InputValidation("Invalid query".to_string());
    let error_debug = format!("{:?}", error);
    assert!(error_debug.contains("InputValidation"));
}
