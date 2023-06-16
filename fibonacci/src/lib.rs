use cached::proc_macro::cached;

/// fib(0) = 0
/// fib(1) = 1
/// fib(n) = fib(n-1) + fib(n-2) for all n >= 0

#[cached] // for memoization
pub fn fib(n: usize) -> usize {
    match n {
        0 | 1 => n,
        _ => fib(n-1) + fib(n-2),
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::fib;

    #[test]
    fn test_fib_numbers() {
        let input_output: Vec<(usize, usize)> = vec![
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 2),
            (4, 3),
            (5, 5),
            (40, 102334155),
        ];
        for (n, exp) in input_output {
            assert_eq!(fib(n), exp);
        }
    }
}
