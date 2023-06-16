/// returns “Fizz” if n is divisible by 3
/// returns “Buzz” if n is divisible by 5
/// returns “FizzBuzz” if n is divisible by both 3 and 5
pub fn fizzbuzz(n: usize) -> String {
    match (n % 3, n % 5) {
        (0, 0) => String::from("FizzBuzz"),
        (0, _) => String::from("Fizz"),
        (_, 0) => String::from("Buzz"),
        _ => String::from(""),
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::fizzbuzz;

    #[test]
    fn test_no_divisible() {
        for i in [1, 2, 4, 7, 8] {
            assert_eq!(fizzbuzz(i), "");
        }
    }

    #[test]
    fn test_divisible_by_3() {
        for i in [3, 6, 9, 12] {
            assert_eq!(fizzbuzz(i), "Fizz");
        }
    }

    #[test]
    fn test_divisible_by_5() {
        for i in [5, 10, 20] {
            assert_eq!(fizzbuzz(i), "Buzz");
        }
    }

    #[test]
    fn test_divisible_by_3_and_5() {
        for i in [0, 15, 30] {
            assert_eq!(fizzbuzz(i), "FizzBuzz");
        }
    }
}
