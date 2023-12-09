use std::env;
use std::fs;
use std::io::{self, BufRead};


/// This function recieves a location of a number string in the matrix that contains the engine schematic.
/// If the value is in contact with a symbol, this function parses the number and returns it, otherwise, 
/// this function returns a 0.
/// 
/// # Arguments
/// 
/// * `lines` - The schematic as a reference to vector of vectors of chars.
/// * `i` - Index of the string that contains the digit sequence that's being checked.
/// * `j_start` - Index where the substring with the sequence begins.
/// * `j_end` - Index where the substring with the sequence ends.
fn check_number(lines: &Vec<Vec<char>>, i: usize, j_from: usize, j_to: usize) -> usize {
    let from_i = if i == 0 { 0 } else { i - 1 };
    let to_i = if i == lines.len() - 1 { i } else { i + 1 };
    let from_j: usize = if j_from == 0 { 0 } else { j_from - 1 };
    let to_j: usize = if j_to == lines[i].len() - 1 { j_to } else { j_to + 1 };

    for x in from_i..=to_i {
        for y in from_j..to_j {
            if !(lines[x][y].is_digit(10) || lines[x][y] == '.') {
                let substring = lines[i][j_from..j_to].iter().collect::<String>();
                return substring.parse::<usize>().unwrap();
            }
        }
    }

    0
}


fn main() {
    let path = env::args().nth(1).expect("Missing required parameter path!");

    let lines: Vec<Vec<char>> = io::BufReader::new(
        fs::File::open(path).expect("Could not open file!"))
        .lines()
        .map(|line| line.expect("Could not read line!").chars().into_iter().collect())
        .collect();

    let mut accumulator: usize = 0;

    for i in 0..lines.len() {
        let mut digits: bool = false;  // are we examining a stream of digits
        let mut first_digit: usize = 0;  // when did that stream start
        for j in 0..lines[i].len() {
            match (digits, lines[i][j].is_digit(10)) {
                (false, true) => {  // starting new stream of digits
                    digits = true;
                    first_digit = j;
                }
                (true, false) => {  // finishing stream of digits
                    accumulator += check_number(&lines, i, first_digit, j);
                    digits = false;
                }
                _ => {}
            }
        }

    }

    println!("Total value of parts: {}", accumulator);
}
