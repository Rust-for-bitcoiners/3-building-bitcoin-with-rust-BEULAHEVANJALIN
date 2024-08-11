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

    fn add<U>(self, res: MResult<U, E>) -> MResult<U, E> {
        match self {
            MResult::Ok(_) => res,
            MResult::Err(e) => MResult::Err(e),
        }
    }

    fn and_then<U, F: FnOnce(T) -> MResult<U, E>>(self, op: F) -> MResult<U, E> {
        match self {
            MResult::Ok(value) => op(value),
            MResult::Err(error) => MResult::Err(error),
        }
    }

    fn as_mut(&mut self) -> MResult<&mut T, &mut E> {
        match self {
            MResult::Ok(ref mut value) => MResult::Ok(value),
            MResult::Err(ref mut err) => MResult::Err(err),
        }
    }

    fn as_ref(&self) -> MResult<&T, &E> {
        match self {
            MResult::Ok(ref value) => MResult::Ok(value),
            MResult::Err(ref err) => MResult::Err(err),
        }
    }

    fn expect(self, msg: &str) -> T {
        match self {
            MResult::Ok(value) => value,
            MResult::Err(_) => panic!("{}", msg),
        }
    }

    fn expect_err(self, msg: &str) -> E {
        match self {
            MResult::Ok(_) => panic!("{}", msg),
            MResult::Err(err) => err,
        }
    }

    fn into_err(self) -> Option<E> {
        match self {
            MResult::Err(err) => Some(err),
            MResult::Ok(_) => None,
        }
    }

    fn into_ok(self) -> Option<T> {
        match self {
            MResult::Ok(value) => Some(value),
            MResult::Err(_) => None,
        }
    }

    fn is_err_and<F: FnOnce(&E) -> bool>(&self, f: F) -> bool {
        match self {
            MResult::Err(err) => f(err),
            MResult::Ok(_) => false,
        }
    }

    fn is_ok_and<F: FnOnce(&T) -> bool>(&self, f: F) -> bool {
        match self {
            MResult::Ok(value) => f(value),
            MResult::Err(_) => false,
        }
    }

    fn iter(&self) -> std::option::IntoIter<&T> {
        match self {
            MResult::Ok(ref value) => Some(value).into_iter(),
            MResult::Err(_) => None.into_iter(),
        }
    }

    fn iter_mut(&mut self) -> std::option::IntoIter<&mut T> {
        match self {
            MResult::Ok(ref mut value) => Some(value).into_iter(),
            MResult::Err(_) => None.into_iter(),
        }
    }

    fn or<U>(self, res: MResult<U, E>) -> MResult<U, E> {
        match self {
            MResult::Ok(_) => res,
            MResult::Err(e) => MResult::Err(e),
        }
    }

    fn or_else<F: FnOnce(E) -> MResult<T, E>>(self, op: F) -> MResult<T, E> {
        match self {
            MResult::Ok(value) => MResult::Ok(value),
            MResult::Err(err) => op(err),
        }
    }

    fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
        match self {
            MResult::Ok(value) => f(value),
            MResult::Err(_) => default,
        }
    }

    fn map_or_else<U, F: FnOnce(T) -> U, D: FnOnce(E) -> U>(self, default: D, f: F) -> U {
        match self {
            MResult::Ok(value) => f(value),
            MResult::Err(err) => default(err),
        }
    }
}


impl<T: Clone, E: Clone> MResult<T, E> {
    fn cloned(self) -> MResult<T, E> {
        match self {
            MResult::Ok(ref value) => MResult::Ok(value.clone()),
            MResult::Err(ref err) => MResult::Err(err.clone()),
        }
    }
}

impl<T: Copy, E: Copy> MResult<T, E> {
    fn copied(self) -> MResult<T, E> {
        match self {
            MResult::Ok(value) => MResult::Ok(value),
            MResult::Err(err) => MResult::Err(err),
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

    #[test]
    fn test_add_ok() {
        let result: MResult<i32, &str> = MResult::ok(42);
        let add_result: MResult<i32, &str> = result.add(MResult::ok(10));
        assert_eq!(add_result, MResult::Ok(10));
    }

    #[test]
    fn test_add_err() {
        let result: MResult<i32, &str> = MResult::err("error");
        let add_result: MResult<i32, &str> = result.add(MResult::ok(10));
        assert_eq!(add_result, MResult::Err("error"));
    }

    #[test]
    fn test_and_then_ok() {
        let result: MResult<i32, &str> = MResult::ok(42);
        let and_then_result: MResult<i32, &str> = result.and_then(|x| MResult::ok(x + 1));
        assert_eq!(and_then_result, MResult::Ok(43));
    }

    #[test]
    fn test_and_then_err() {
        let result: MResult<i32, &str> = MResult::err("error");
        let and_then_result: MResult<i32, &str> = result.and_then(|x| MResult::ok(x + 1));
        assert_eq!(and_then_result, MResult::Err("error"));
    }

    #[test]
    fn test_expect_ok() {
        let result: MResult<i32, &str> = MResult::ok(42);
        assert_eq!(result.expect("Unexpected error"), 42);
    }

    #[test]
    #[should_panic(expected = "Expected an Ok value")]
    fn test_expect_err() {
        let result: MResult<i32, &str> = MResult::err("error");
        result.expect("Expected an Ok value");
    }

    #[test]
    fn test_into_ok() {
        let result: MResult<i32, &str> = MResult::ok(42);
        assert_eq!(result.into_ok(), Some(42));
    }

    #[test]
    fn test_into_err() {
        let result: MResult<i32, &str> = MResult::err("error");
        assert_eq!(result.into_err(), Some("error"));
    }

    #[test]
    fn test_is_ok_and_true() {
        let result: MResult<i32, &str> = MResult::ok(42);
        assert!(result.is_ok_and(|&x| x > 40));
    }

    #[test]
    fn test_is_ok_and_false() {
        let result: MResult<i32, &str> = MResult::ok(42);
        assert!(!result.is_ok_and(|&x| x < 40));
    }

    #[test]
    fn test_is_err_and_true() {
        let result: MResult<i32, &str> = MResult::err("error");
        assert!(result.is_err_and(|&e| e == "error"));
    }

    #[test]
    fn test_is_err_and_false() {
        let result: MResult<i32, &str> = MResult::err("error");
        assert!(!result.is_err_and(|&e| e == "not an error"));
    }

    #[test]
    fn test_map_or_ok() {
        let result: MResult<i32, &str> = MResult::ok(42);
        let mapped_result = result.map_or(0, |x| x + 1);
        assert_eq!(mapped_result, 43);
    }

    #[test]
    fn test_map_or_err() {
        let result: MResult<i32, &str> = MResult::err("error");
        let mapped_result = result.map_or(0, |x| x + 1);
        assert_eq!(mapped_result, 0);
    }

    #[test]
    fn test_map_or_else_ok() {
        let result: MResult<i32, &str> = MResult::ok(42);
        let mapped_result = result.map_or_else(|_| 0, |x| x + 1);
        assert_eq!(mapped_result, 43);
    }

    #[test]
    fn test_map_or_else_err() {
        let result: MResult<i32, &str> = MResult::err("error");
        let mapped_result = result.map_or_else(|_| 0, |x| x + 1);
        assert_eq!(mapped_result, 0);
    }

}