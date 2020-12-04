use std::borrow::{Borrow, BorrowMut};
use std::char::from_u32;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::io;
use std::time::Instant;

#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;
fn main() -> io::Result<()> {
    let now = Instant::now();
    let input = read_input("src/input.txt");
    println!("solution part1: {}", count_valid_passports(input, is_passport_valid));
    println!("part 1 took {}ms", now.elapsed().as_millis());

    let now2 = Instant::now();
    let input2 = read_input("src/input.txt");
    println!("solution part2: {}", count_valid_passports(input2, has_all_valid_fields));
    println!("part 2 took {}ms", now2.elapsed().as_millis());

    Ok(())
}

fn count_valid_passports(passports: Vec<HashMap<String, String>>, validation_function: fn(&HashMap<String, String>) -> bool) -> i32 {
    let mut num_pp = 0;
    for passport in passports {
        print!(" passport: {:?}", passport);
        if validation_function(&passport) {
            num_pp += 1;
            println!("===is VALID")
        } else {
            println!("===is IN-VALID")
        }
    }
    return num_pp;
}


fn has_all_valid_fields(passport_fields: &HashMap<String, String>) -> bool {
    let exceptions: Vec<String> = vec![String::from("cid")];

    for field in passport_fields {
        let is_valid = validate_field(field);
        println!("validating field {:?} {}", field, is_valid);
        if is_valid == false { return false; }
    }
    let length = passport_fields.keys().len();
    return length == 8 || (length == 7 && !passport_fields.contains_key(exceptions.get(0).unwrap()));
}


fn is_passport_valid(passport_fields: &HashMap<String, String>) -> bool {
    let exceptions: Vec<String> = vec![String::from("cid")];
    println!("{}", passport_fields.keys().len());
    return passport_fields.keys().len() == 8 || (passport_fields.keys().len() == 7 && !passport_fields.contains_key(exceptions.get(0).unwrap()));
}

fn validate_field(field: (&String, &String)) -> bool {
    match field.0.as_str() {
        "byr" => {
            lazy_static! {
            static ref byr_regex:Regex = Regex::new(r"\d{4}").unwrap();
            }
            if !byr_regex.is_match(field.1.as_str()) || field.1.parse::<i32>().ok().unwrap() < 1920 || field.1.parse::<i32>().ok().unwrap() > 2002 {
                return false;
            }
        }
        "iyr" => {
            lazy_static! {
            static ref iyr_regex:Regex = Regex::new(r"\d{4}").unwrap();
            }
            if !iyr_regex.is_match(field.1.as_str()) || field.1.parse::<i32>().ok().unwrap() < 2010 || field.1.parse::<i32>().ok().unwrap() > 2020 {
                return false;
            }
        }
        "eyr" => {
            lazy_static!{
            static ref eyr_regex:Regex = Regex::new(r"\d{4}").unwrap();
            }
            if !eyr_regex.is_match(field.1.as_str()) || field.1.parse::<i32>().ok().unwrap() < 2020 || field.1.parse::<i32>().ok().unwrap() > 2030 {
                return false;
            }
        }
        "hgt" => {
            lazy_static!{
            static ref hgt_regex:Regex = Regex::new(r"(\d{2,3})(cm|in)").unwrap();
            }
            if hgt_regex.is_match(field.1.as_str()) {
                let height = hgt_regex.captures(field.1.as_str()).unwrap();
                if height[2] == *"cm" && (height[1].parse::<i32>().unwrap() < 150 || height[1].parse::<i32>().unwrap() > 193) {
                    return false;
                }
                if height[2] == *"in" && (height[1].parse::<i32>().unwrap() < 59 || height[1].parse::<i32>().unwrap() > 76) {
                    return false;
                }
                return true;
            }
            return false;
        }
        "hcl" => {
            lazy_static!{
            static ref hcl_regex:Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            }
            if !hcl_regex.is_match(field.1.as_str()) {
                return false;
            }
        }
        "ecl" => {
            lazy_static!{
            static ref ecl_regex:Regex = Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
            }
            if !ecl_regex.is_match(field.1.as_str()) {
                return false;
            }
        }
        "pid" => {
            lazy_static!{
            static ref pid_regex:Regex = Regex::new(r"^\d{9}$").unwrap();
            }
            if !pid_regex.is_match(field.1.as_str()) {
                return false;
            }
        }
        _ => { return true; }
    }
    return true;
}

fn read_input(filename_path: &str) -> Vec<HashMap<String, String>> {
    let file = File::open(filename_path).unwrap();
    let mut reader = BufReader::new(file);
    let mut lines = reader.by_ref().lines();
    let mut current_passport: HashMap<String, String> = Default::default();
    let mut passports: Vec<HashMap<String, String>> = vec![];
    let mut num_empty_lines = 0;
    for line in lines {
        let line_string = line.ok().unwrap();
        if line_string.is_empty() {
            passports.push(current_passport);
            current_passport = Default::default();
            num_empty_lines += 1;
        }
        for entry in line_string.split_ascii_whitespace() {//split "key:value key2:value2"
            let parts: Vec<&str> = entry.split(":").collect();
            current_passport.insert(String::from(parts[0]), String::from(parts[1]));
        }
    }
    passports.push(current_passport);// just because I can't seemingly figure out how to make peek() work X-D
    println!("empty lines:{}", num_empty_lines);
    println!("num passports:{}", passports.len());
    return passports;
}

#[cfg(test)]
mod tests {
    use std::io;

    use crate::{count_valid_passports, is_passport_valid, read_input, validate_field};

    #[test]
    fn test_from_example() -> io::Result<()> {//input and output given by the example output in the puzzle
        let solution = 2;
        assert_eq!(solution, count_valid_passports(read_input("src/test.txt"), is_passport_valid));
        Ok(())
    }

    #[test]
    fn test_byr_validation() -> io::Result<()> {//input and output given by the example output in the puzzle
        assert_eq!(false, validate_field((&String::from("byr"), &String::from("2020"))));
        assert_eq!(true, validate_field((&String::from("byr"), &String::from("2002"))));
        Ok(())
    }

    #[test]
    fn test_hgt_in_validation() -> io::Result<()> {
        assert_eq!(true, validate_field((&String::from("hgt"), &String::from("60in"))));
        assert_eq!(false, validate_field((&String::from("hgt"), &String::from("100in"))));
        Ok(())
    }

    #[test]
    fn test_hgt_cm_validation() -> io::Result<()> {
        assert_eq!(true, validate_field((&String::from("hgt"), &String::from("180cm"))));
        assert_eq!(false, validate_field((&String::from("hgt"), &String::from("10cm"))));
        assert_eq!(false, validate_field((&String::from("hgt"), &String::from("200cm"))));
        Ok(())
    }

    #[test]
    fn test_hcl_validation() -> io::Result<()> {
        assert_eq!(true, validate_field((&String::from("hcl"), &String::from("#ffffff"))));
        assert_eq!(false, validate_field((&String::from("hcl"), &String::from("ffffff"))));
        assert_eq!(false, validate_field((&String::from("hcl"), &String::from("#abcdef1"))));
        Ok(())
    }
}
