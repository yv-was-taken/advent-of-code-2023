use std::fs::File;
use std::io::{self, BufRead};

// Function to extract the first and last numbers from a line and convert them to u32
fn extract_first_last_numbers(line: &str) -> u32 {
    let numbers: Vec<_> = line.chars().filter(|c| c.is_digit(10)).collect();
    let first = numbers.get(0).unwrap_or(&'0');
    let last = numbers.last().unwrap_or(&'0');

    let result_str = format!("{}{}", first, last);

    // Parse the result string into a u32
    result_str.parse().unwrap_or(0)
}

fn main() -> io::Result<()> {
    // Replace "your_file.txt" with the actual path to your file
    let file_path = "./codes.txt";

    // Read the contents of the file
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);

    let result: Vec<u32> = reader
        .lines()
        .filter_map(|line| line.ok()) // Handle potential errors with lines()
        .map(|line| extract_first_last_numbers(&line))
        .collect();

//    // Print the result
//    println!("{:?}", result);

    let sum:u32 = result.iter().sum();

    println!("sum: {}", sum);

    Ok(())
}

