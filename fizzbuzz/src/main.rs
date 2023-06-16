use fizzbuzz::fizzbuzz;

fn main() {
    for i in [1, 3, 5, 15] {
        println!("fizzbuzz({}) -> {}", i, fizzbuzz(i));
    }
}
