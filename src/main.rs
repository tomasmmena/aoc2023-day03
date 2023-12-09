use std::env;
use std::fs;
use std::io::{self, BufRead};


fn explore_number(lines: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let mut from_j = j;
    let mut to_j = j;

    while from_j > 0 {
        if !lines[i][from_j - 1].is_digit(10) {
            break;
        }
        from_j -= 1;
    }

    while to_j < lines[i].len() - 1 {
        if !lines[i][to_j + 1].is_digit(10) {
            break;
        }
        to_j += 1;
    }
    let substring = lines[i][from_j..=to_j].iter().collect::<String>();
    substring.parse::<usize>().unwrap()
}


/// This function recieves a location of a gear in the schematic and looks for numbers around it to 
/// calculate its ratio. If less or more than 2 numbers are found, it returns 0 instead.
/// 
/// # Arguments
/// 
/// * `lines` - The schematic as a reference to vector of vectors of chars.
/// * `i` - Index of the string that contains the gear that's being checked.
/// * `j` - Index where the gear is located in the substring.
fn check_ratio(lines: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let from_i = if i == 0 { 0 } else { i - 1 };
    let to_i = if i == lines.len() - 1 { i } else { i + 1 };

    let mut numbers: Vec<usize> = vec![];
    for x in from_i..=to_i {
        if lines[x][j].is_digit(10) {
            numbers.push(explore_number(lines, x, j))
        } else {
            if j > 0 && lines[x][j - 1].is_digit(10) {
                numbers.push(explore_number(lines, x, j - 1))
            }
            if j < lines[x].len() - 1 && lines[x][j + 1].is_digit(10) {
                numbers.push(explore_number(lines, x, j + 1))
            }
        }
    }

    if numbers.len() == 2 {
        return numbers[0] * numbers[1]
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
        for j in 0..lines[i].len() {
            if lines[i][j] == '*' {
                accumulator += check_ratio(&lines, i, j);
            }
        }

    }

    println!("Total value of ratios: {}", accumulator);
}
