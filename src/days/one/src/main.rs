use std::fs::File;
use std::io::{self, BufRead};
//
//part one
//// Function to extract the first and last numbers from a line and convert them to u32
//fn extract_first_last_numbers(line: &str) -> u32 {
//    let numbers: Vec<_> = line.chars().filter(|c| c.is_digit(10)).collect();
//    let first = numbers.get(0).unwrap_or(&'0');
//    let last = numbers.last().unwrap_or(&'0');
//
//    let result_str = format!("{}{}", first, last);
//
//    // Parse the result string into a u32
//    result_str.parse().unwrap_or(0)
//}
//
//fn main() -> io::Result<()> {
//    let file_path = "./codes.txt";
//
//    // Read the contents of the file
//    let file = File::open(&file_path)?;
//    let reader = io::BufReader::new(file);
//
//    let result: Vec<u32> = reader
//        .lines()
//        .filter_map(|line| line.ok()) // Handle potential errors with lines()
//        .map(|line| extract_first_last_numbers(&line))
//        .collect();
//
////    // Print the result
////    println!("{:?}", result);
//
//    let sum:u32 = result.iter().sum();
//
//    println!("sum: {}", sum);
//
//    Ok(())
//}


//part two
fn extract_first_last_numbers(line: &str) -> u32 {
    let num_strings = vec!["one", "two", "three", "four", "five","six", "seven", "eight", "nine"];
    let does_line_contain_num_strings = line.chars().scan(String::new(), |acc, c| {
        acc.push(c);
        Some(acc.clone())
    }).any(|potential_num_string| num_strings.contains(&potential_num_string.as_str()));

    if does_line_contain_num_strings {


    } else {

    }
    //if does_line_contain_num_strings {
    //    let mut potential_num_string: String = String::new();
    //    let characters: Vec<_> = line.chars().iter().:


    //}
    let numbers: Vec<_> = line.chars().filter(|c| c.is_digit(10)).collect();
    let first = numbers.get(0).unwrap_or(&'0');
    let last = numbers.last().unwrap_or(&'0');

    let result_str = format!("{}{}", first, last);

    // Parse the result string into a u32
    result_str.parse().unwrap_or(0)
}



//@dev note to future self
//commenting out the main fn that actually iterates through the file because I just want to see if
//it actually does replace the numstring with the num for each line
//after that we're home free
//intended behaviour of the fn is to iterate through each line and return the before and after
//parsing the line, supposed to replace each num string with num, 
//but fn not working atm
//ttfn!
fn main() {
    let num_strings = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut line = String::from("onetwothree");

    let replaced_line: String = line
        .chars()
        .enumerate()
        .scan(String::new(), |acc, (index, c)| {
            acc.push(c);
            Some((index, acc.clone()))
        })
    .filter_map(|(index, potential_num)| {
        if num_strings.contains(&potential_num.as_str()) {
            Some(index.checked_sub(potential_num.len())?.checked_add(1)?)
        } else {
            None
        }
    })
    .next()
        .map(|start| {
            let end = start + num_strings[0].len();
            line.replace_range(start..end, "1"); // Replace with the integer version (1 in this case)
        line.clone()
        })
    .unwrap_or(line.clone());

    println!("Original line: {}", line);
    println!("Replaced line: {}", replaced_line);
}
//fn main() -> io::Result<()> {
//    let file_path = "./codes.txt";
//
//    // Read the contents of the file
//    let file = File::open(&file_path)?;
//    let reader = io::BufReader::new(file);
//
//    let result: Vec<u32> = reader
//        .lines()
//        .filter_map(|line| line.ok()) // Handle potential errors with lines()
//        .map(|line| extract_first_last_numbers(&line))
//        .collect();
//
//    //    // Print the result
//    //    println!("{:?}", result);
//
//    let sum:u32 = result.iter().sum();
//
//    println!("sum: {}", sum);
//
//    Ok(())
//}
