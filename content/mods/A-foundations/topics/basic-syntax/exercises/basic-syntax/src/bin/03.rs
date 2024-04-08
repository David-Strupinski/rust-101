fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    let (largest, smallest) = find_largest_and_smallest(input);

    println!("{} is largest and {} is smallest", largest, smallest);
}

fn find_largest_and_smallest(input: [i32; 8]) -> (i32, i32) {
    let mut largest = input[0];
    let mut smallest = input[0];

    for &n in &input {
        if n > largest {
            largest = n;
        }
        if n < smallest {
            smallest = n;
        }
    }

    (largest, smallest)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_largest_and_smallest() {
        assert_eq!(find_largest_and_smallest([23, 82, 16, 45, 21, 94, 12, 34]), (94, 12));
        assert_eq!(find_largest_and_smallest([1, 2, 3, 4, 5, 6, 7, 8]), (8, 1));
        assert_eq!(find_largest_and_smallest([1, 2, 3, 4, 5, 6, 7, 8]), (8, 1));
    }
}
