use std::fs::File;
use std::io::{BufReader, Read, BufRead};
use std::collections::HashSet;

fn main() {
    //i really need to start using refs X-)
    let input = read_input("src/input.txt");
    let input2 = read_input("src/input.txt");

    count_distinct_answers_per_group(input);
    count_answers_everyone_yes(input2);
}

fn read_input(filename_path: &str) -> Vec<String> {
    let file = File::open(filename_path).unwrap();
    let mut reader = BufReader::new(file);
    let lines = reader.by_ref().lines();
    let input_vec: Vec<String> = lines.map(|l| l.ok().unwrap()).collect();
    return input_vec;
}

fn count_distinct_answers_per_group(lines: Vec<String>) -> usize{
    let mut answers: HashSet<char> = HashSet::new();
    let mut sum: usize = 0;
    for line in lines.iter() {
        answers.extend(line.chars());
        if line.is_empty() {
            println!("answers: {}", answers.len());
            sum += answers.len();
            answers.clear();
        }
    }
    println!("end sum: {}", &sum);
    return sum;
}

fn count_answers_everyone_yes(lines: Vec<String>) -> usize{
    let mut answers: HashSet<char> = HashSet::new();
    let mut current_answers:HashSet<char> = HashSet::new();
    let mut sum: usize = 0;
    let mut first_after_nl:bool = true;
    for line in lines.iter() {
        if line.is_empty() {
            println!("answers: {}", answers.len());
            sum += answers.len();
            answers.clear();
            first_after_nl=true;
        }else {
            current_answers.clear();
            current_answers.extend(line.chars());
            println!("current line {:?}", &current_answers);
            let intersection: HashSet<char> = answers.intersection(&current_answers).copied().collect();
            answers.clear();
            print!("adding ");
            if first_after_nl {
                println!("{:?}", &current_answers);
                answers.extend(&current_answers);
                first_after_nl = false;
            } else {
                println!("intersection: {:?}", &intersection);
                answers.extend(&intersection);
            }
        }


    }
    println!("end sum: {}", &sum);
    return sum;
}

#[cfg(test)]
mod tests {
    use std::io;

    use crate::{read_input, count_distinct_answers_per_group, count_answers_everyone_yes};

    #[test]
    fn test_from_example() -> io::Result<()> {//input and output given by the example output in the puzzle
        let solution = 11;
        assert_eq!(solution, count_distinct_answers_per_group(read_input("src/test.txt")));
        Ok(())
    }
    #[test]
    fn test_from_example2() -> io::Result<()> {//input and output given by the example output in the puzzle
        let solution = 6;
        assert_eq!(solution, count_answers_everyone_yes(read_input("src/test.txt")));
        Ok(())
    }
}
