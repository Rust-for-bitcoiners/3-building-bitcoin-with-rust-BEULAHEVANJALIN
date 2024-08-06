#![allow(unused)]

#[derive(Debug, PartialEq)]

enum MResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> MResult<T, E> {
    fn ok(value: T) -> Self {
        MResult::Ok(value)
    }
    // Function to create an Err variant
    fn err(error: E) -> Self {
        MResult::Err(error)
    }

    // Method to check if it's an Ok variant
    fn is_ok(&self) -> bool {
        matches!(self, MResult::Ok(_))
    }

    // Method to check if it's an Err variant
    fn is_err(&self) -> bool {
        matches!(self, MResult::Err(_))
    }

    // Method to unwrap the Ok value, panics if it's an Err
    fn unwrap(self) -> T {
        match self {
         MResult::Ok(value) => value,
         MResult::Err(_) => panic!("panics as it's an Err"),
        }
    }

    // Method to unwrap the Err value, panics if it's an Ok
    fn unwrap_err(self) -> E {
        match self {
            MResult::Ok(_) => panic!("panics as it's an Ok"),
            MResult::Err(error) => error
        }
    }
}

// Add unit tests below
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let result: MResult<i32, &str> = MResult::ok(42);
        assert_eq!(result, MResult::Ok(42));
    }

    #[test]
    fn test_err() {
        let result: MResult<i32, &str> = MResult::err("error");
        assert_eq!(result, MResult::Err("error"));
    }

    #[test]
    fn test_is_ok_true() {
        let result: MResult<i32, &str> = MResult::ok(42);
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_ok_false() {
        let result: MResult<i32, &str> = MResult::err("error");
        assert!(!result.is_ok());
    }

    #[test]
    fn test_is_err_true() {
        let result: MResult<i32, &str> = MResult::err("error");
        assert!(result.is_err());
    }

    #[test]
    fn test_is_err_false() {
        let result: MResult<i32, &str> = MResult::ok(42);
        assert!(!result.is_err());
    }

    #[test]
    fn test_unwrap_ok() {
        let result: MResult<i32, &str> = MResult::ok(42);
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    #[should_panic(expected = "panics as it's an Err")]
    fn test_unwrap_err_variant() {
        let result: MResult<i32, &str> = MResult::err("error");
        result.unwrap();
    }

    #[test]
    fn test_unwrap_err() {
        let result: MResult<i32, &str> = MResult::err("error");
        assert_eq!(result.unwrap_err(), "error");
    }

    #[test]
    #[should_panic(expected = "panics as it's an Ok")]
    fn test_unwrap_err_on_ok_variant() {
        let result: MResult<i32, &str> = MResult::ok(42);
        result.unwrap_err();
    }

}