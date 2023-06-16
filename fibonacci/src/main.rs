use fibonacci::fib;

fn main() {
    for i in [0, 1, 2, 3, 4, 5, 40] {
        println!("fibonacci({}) => {}", i, fib(i));
    }
}
