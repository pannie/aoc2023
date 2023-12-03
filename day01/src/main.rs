use std::fs::read_to_string;
fn main() {
    let valid_numbers = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let lines = read_lines("./input/input.txt");

    let answer: u32 = lines.iter().map(|line| {
        let number_left = valid_numbers
            .iter()
            .enumerate()
            .map(|(valid_number_index, number)| (valid_number_index, line.find(number)))
            .filter(|(_, index_found)| index_found.is_some())
            .map(|(valid_number_index, index_found)| (valid_number_index, index_found.expect("expect some")))
            .min_by_key(|(_, index_found)| index_found.clone())
            .map(|(valid_number_index, _)| valid_number_index % 9 + 1)
            .expect("expect a value");

        let number_right = valid_numbers
            .iter()
            .enumerate()
            .map(|(valid_number_index, number)| (valid_number_index, line.rfind(number)))
            .filter(|(_, index_found)| index_found.is_some())
            .map(|(valid_number_index, index_found)| (valid_number_index, index_found.expect("expect some")))
            .min_by_key(|(_, index_found)| index_found.clone())
            .map(|(valid_number_index, _)| valid_number_index % 9 + 1)
            .expect("expect a value");

        (number_left * 10 + number_right) as u32
    }).sum();

    println!("{answer}")
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}