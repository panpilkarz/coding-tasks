```
$ time cargo run
(...)
fibonacci(0) => 0
fibonacci(1) => 1
fibonacci(2) => 1
fibonacci(3) => 2
fibonacci(4) => 3
fibonacci(5) => 5
fibonacci(40) => 102334155

real    0m0.211s
user    0m0.065s
sys     0m0.024s
```

```
$ cargo test --lib
(...)

running 1 test
test tests::test_fib_numbers ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
