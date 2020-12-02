use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

use regex::Regex;

fn main() -> io::Result<()> {
    let input = read_input("src/day02.txt");
    let input2 = read_input("src/day02.txt");
    println!("solution part1: {}", count_compliant_lines(parse_input(input),is_compliant));
    println!("solution part2: {}", count_compliant_lines(parse_input(input2),is_compliant_positions));
    println!("execution finished");

    Ok(())
}

fn read_input(filename_path: &str) -> Vec<String> {
    let file = File::open(filename_path).unwrap();
    let mut reader = BufReader::new(file);
    let lines = reader.by_ref().lines();
    let input_vec: Vec<String> = lines.map(|l| l.ok().unwrap()).collect();
    return input_vec;
}

#[derive(Debug)]
struct PwEntry {
    min: i32,
    max: i32,
    letter: char,
    pw: String,
}

fn count_compliant_lines(entries: Vec<PwEntry>, compliance_function:fn(PwEntry) ->bool) -> i32 {
    let mut num_compliant_entries = 0;
    for entry in entries {
        if compliance_function(entry) {
            num_compliant_entries += 1;
        }
    }
    return num_compliant_entries;
}

fn parse_input(input: Vec<String>) -> Vec<PwEntry> {
    let mut parsed_input: Vec<PwEntry> = vec![];
    let re = Regex::new(r"(\d{1,2})-(\d{1,2})\s([a-z]{1}):\s(.*)").unwrap();

    for line in input {
        for cap in re.captures_iter(&line) {
            println!("min: {} max: {} letter: {}, string {}", &cap[1], &cap[2], &cap[3], &cap[4]);
            let entry: PwEntry = PwEntry {
                min: (&cap[1]).parse().unwrap(),
                max: (&cap[2]).parse().unwrap(),
                letter: (&cap[3]).parse().unwrap(),
                pw: String::from(&cap[4]),
            };
            parsed_input.push(entry);
        }
    }
    return parsed_input;
}

fn is_compliant(entry: PwEntry) -> bool {
    let mut number_of_occurences: i32 = 0;
    for current_char in entry.pw.chars() {
        if current_char == entry.letter {
            number_of_occurences += 1;
        }
    }
    return number_of_occurences >= entry.min && number_of_occurences <= entry.max;
}
fn is_compliant_positions(entry: PwEntry) -> bool {
    let mut chars = entry.pw.chars();
    let mut chars2 = entry.pw.chars();
    //xor because exactly one must match
    return (chars.nth((entry.min - 1) as usize).unwrap()==entry.letter)
            ^ (chars2.nth((entry.max - 1) as usize).unwrap()==entry.letter);
}
#[cfg(test)]
mod tests {
    use std::io;
    use crate::{count_compliant_lines, parse_input, read_input, is_compliant};

    #[test]
    fn test_from_example() -> io::Result<()> {//input and output given by the example output in the puzzle

        let solution = 2;
        assert_eq!(solution, count_compliant_lines(parse_input(read_input("src/test02.txt")),is_compliant));
        Ok(())
    }
}
