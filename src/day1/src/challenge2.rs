use std::collections::HashSet;
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

fn get_unique_numbers_in_array(array: &[i32]) -> HashSet<i32> {
    let unique_numbers: HashSet<i32> = array.iter().cloned().collect();
    unique_numbers
}

fn count_number_of_ocurrences_in_array(array: &[i32], number: i32) -> i32 {
    let count: i32 = array.iter().filter(|&n| *n == number).count() as i32;
    count
}

fn main() {
    const FILENAME: &str = "src/day1/input.txt";
    match read_arrays_from_file(FILENAME) {
        Ok((array1, array2)) => {
            let unique_numbers: HashSet<i32> = get_unique_numbers_in_array(&array1);
            let mut similarity_score: i32 = 0;
            for number in unique_numbers {
                let count: i32 = count_number_of_ocurrences_in_array(&array2, number);
                similarity_score += number * count
            }
            println!("Similarity score: {}", similarity_score);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}
