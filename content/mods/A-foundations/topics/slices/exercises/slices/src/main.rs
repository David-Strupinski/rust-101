// This a unfinished implementation of the well-known merge sort algorithm
//
// 1. Fix the language problems in the function merge
//
// 2. Finish the implementation of the function merge_sort
//
// 3. EXTRA: try changing the type from String into String everywhere; does your program still compile? What changes are necessary?

/// Merge two array slices (that have to be sorted) into a vector
fn merge(a: &[String], b: &[String]) -> Vec<String> {
    let mut dest = Vec::new();

    let mut a_idx = 0;
    let mut b_idx = 0;

    while a_idx < a.len() && b_idx < b.len() {
        if a[a_idx] <= b[b_idx] {
            dest.push((a[a_idx].clone()).to_string());
            a_idx += 1
        } else {
            dest.push((b[b_idx].clone()).to_string());
            b_idx += 1
        }
    }

    for elem in &a[a_idx..] {
        dest.push((*elem.clone()).to_string())
    }
    for elem in &b[b_idx..] {
        dest.push((*elem.clone()).to_string())
    }

    dest
}

/// Take an array slice, and sort into a freshly constructed vector using the above function
fn merge_sort(data: &[String]) -> Vec<String> {
    if data.len() > 1 {
        let m = data.len() / 2;
        let left = merge_sort(&data[..m]);
        let right = merge_sort(&data[m..]);
        merge(&left, &right)
    } else {
        data.to_vec()
    }
}

/// Read a bunch of numbers from standard input into a Vec<String>.
fn read_numbers() -> Vec<String> {
    use std::io;
    println!("Please enter some numbers, one or more per line separated by spaces. End with Ctrl-D (Linux) or Ctrl-Z (Windows).");
    let mut result = Vec::new();
    for line in io::stdin().lines().flatten() {
        for word in line.split_whitespace() {
            result.push(word.parse().unwrap())
        }
    }

    result
}

fn main() {
    let input = read_numbers();
    println!("Data to be sorted:");
    println!("{input:?}");

    let sorted_input = merge_sort(&input);
    println!("Sorted data:");
    println!("{sorted_input:?}");
}

// you can run these automatic tests by typing 'cargo test'
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort() {
	assert_eq!(merge_sort(&[]), vec![]);
	assert_eq!(merge_sort(&[5]), vec![5]);
	assert_eq!(merge_sort(&[1,2,3]), vec![1,2,3]);
	assert_eq!(merge_sort(&[47,42,5,1]), vec![1,5,42,47]);
	assert_eq!(merge_sort(&[6,47,42,5,1,123]), vec![1,5,6,42,47,123]);
    }
}
