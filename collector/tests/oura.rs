#[path = "../src/base_integration.rs"] mod base_integration;
use crate::base_integration::Oura;
use crate::base_integration::BaseIntegration;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_oura() {
        let oura = Oura::new();
        assert_eq!(oura.name(), "Oura");
        assert_eq!(oura.authorize(), true);
        assert_eq!(oura.get_data("2020-01-01".to_string(), "2020-01-01".to_string()), "Oura data");
    }


    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 3);
    }
}