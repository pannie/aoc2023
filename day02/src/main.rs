use std::fs::read_to_string;

fn main() {
    let input = read_lines("./input/input.txt");

    let result: Vec<_> = input
        .iter()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(game_part , color_part)| (&game_part[5..], color_part))


        .collect();

    println!("{:?}", result)
}

struct Game {
    id: u32,
    red: u32,
    blue: u32,
    green: u32,
}
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

