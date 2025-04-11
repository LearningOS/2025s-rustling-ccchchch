pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        // Test with an even number (e.g., 4)
        assert!(is_even(4)); // The function should return true for even numbers
    }

    #[test]
    fn is_false_when_odd() {
        // Test with an odd number (e.g., 5)
        assert!(!is_even(5)); // The function should return false for odd numbers
    }
}
