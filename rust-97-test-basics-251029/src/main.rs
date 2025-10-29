fn main() {
   add_two(3);
}

pub fn compare_vals(x: i32, y: i32) -> bool {
    x == y
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}

fn greeting(name: &str) -> String {
    format!("Hello")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_equal() {
        let result = compare_vals(20, 20);
        // The simplest test is using the assert!() macro
        // This passes when its parameter is true and fails when false
        assert!(result);
    }

    #[test]
    fn adds_two() {
        let result = add_two(3);
        // A more precise test is the assert_eq!() macro
        // This passes when both parameters are equal
        // When they are not, the fail shows the two values
        assert_eq!(result, 5);
    }

    #[test]
    fn is_not_input() {
        let input = 3;
        let result = add_two(input);
        // There is also an assert_ne!() macro
        // It passes the test, when poth parameters are not equal
        assert_ne!(result, input);
    }

    #[test]
    fn contains_name() {
        let result = greeting("Name");
        // When you place a format!() style string after the parameters the assert macros need
        // It prints additional text when the test fails to provide more detail
        assert!(
            result.contains("Name"),
            "The name was not found in greeting, value was {result}"
        );
    }
}