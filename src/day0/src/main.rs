use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_arrays_from_file(filename: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let path: &Path = Path::new(filename);
    let file: File = File::open(path)?;
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let mut array1: Vec<i32> = vec![];
    let mut array2: Vec<i32> = vec![];

    for line in reader.lines() {
        let line: String = line?;

        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s: &str| s.parse().unwrap())
            .collect();
        let value1: i32 = values[0];
        let value2: i32 = values[1];
        array1.push(value1);
        array2.push(value2);
    }
    Ok((array1, array2))
}

fn sort_array(array: &mut [i32]) {
    array.sort();
}

fn calculate_difference_at_index(array1: &[i32], array2: &[i32], index: usize) -> Option<i32> {
    if index < array1.len() && index < array2.len() {
        Some((array1[index] - array2[index]).abs())
    } else {
        None
    }
}

fn main() {
    match read_arrays_from_file("src/day0/src/input.txt") {
        Ok((mut array1, mut array2)) => {
            sort_array(&mut array1);
            sort_array(&mut array2);

            let mut sum_of_differences: i32 = 0;
            for i in 0..array1.len().min(array2.len()) {
                if let Some(diff) = calculate_difference_at_index(&array1, &array2, i) {
                    println!("Difference at index {}", diff);
                    sum_of_differences += diff;
                }
            }
            println!("Sum of differences: {}", sum_of_differences);
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}
