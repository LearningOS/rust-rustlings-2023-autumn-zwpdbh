// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    (1..=num).fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
    #[test]
    fn my_map_reduce_01() {
        let strings = vec!["rust", "is", "awesome"];

        // Uppercase each string and concatenate them using the map and fold pattern
        let result: String = strings
            .into_iter() // Convert the vector into an iterator
            .map(|s| s.to_uppercase()) // Uppercase each string
            .fold(String::new(), |acc, s| {
                if acc.is_empty() {
                    s
                } else {
                    acc + " " + &s
                }
            }); // Concatenate the strings
        assert_eq!(result, "RUST IS AWESOME")
    }
}
