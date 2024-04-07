fn main() {
    // let mut x: u64 = 0;
    for i in 0..20 {
        println!("Hello, world! fib({}) is {}", i, fib(i));
    }
}

fn fib(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
