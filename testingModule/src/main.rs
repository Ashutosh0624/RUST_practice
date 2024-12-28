// Function to calculate the sum of two integers
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

// Main function
fn main() {
    let a: i32 = 90;
    let b: i32 = 88;
    let c = sum(a, b);
    println!("{}", c);
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*; // Import the outer scope (for sum function)

    #[test]
    fn test_sum_positive_numbers() {
        assert_eq!(sum(2, 3), 5); // Passes
        assert_eq!(sum(10, 20), 30); // Passes
    }

    #[test]
    fn test_sum_negative_numbers() {
        assert_eq!(sum(-2, -3), -5); // Passes
        assert_eq!(sum(-10, 5), -5); // Passes
    }

    #[test]
    fn test_sum_zero() {
        assert_eq!(sum(0, 0), 0); // Passes
        assert_eq!(sum(0, 5), 5); // Passes
    }

    #[test]
    #[should_panic]
    fn test_sum_overflow() {
        // This test ensures that the function handles overflow correctly.
        // Will panic if the sum exceeds i32::MAX.
        sum(i32::MAX, 1); // Should cause an overflow
    }
}
