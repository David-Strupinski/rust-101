fn main() {
    for n in [10, 20, 30, 40] {
        let mult = if n < 25 {
            n * 4
        } else {
            // ending in semicolon makes it an expression, which returns a unit
            n * 3
        };
        println!("{}", mult);
    }
}
