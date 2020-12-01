use std::fs::{File, read};
use std::io;
use std::io::{BufReader, BufRead, Read};

fn main() -> io::Result<()>{

    let file = File::open("src/day01.txt")?;
    let mut reader = BufReader::new(file);
    let lines = reader.by_ref().lines();
    let input_vec:Vec<i32> = lines.map(|i| i.ok().unwrap().parse().unwrap()).collect();

    println!("solution part1: {}", find_sum_and_multiply(&input_vec, 2020));
    println!("solution part2: {}", find_sum_and_multiply_several(&input_vec, 2020));
    println!("execution finished");

    Ok(())
}

fn find_sum_and_multiply(input:&Vec<i32>, sum_to_find:i32) -> i32{
    //let's try a naive solution first
    for i in input {
        let candidate1:i32 = *i;
        for j in input {
            let candidate2:i32 = *j;
            //println!("candidate2:{}",candidate2);
            println!("candidate1:{} candidate2:{},added:{}",candidate1,candidate2,candidate1+candidate2);
            if candidate1+candidate2 == sum_to_find {
                println!("solution part 1: {}",candidate1*candidate2);
                return candidate1*candidate2;
            }
        }
    }
    return -1;
}
fn find_sum_and_multiply_several(input:&Vec<i32>, sum_to_find:i32) -> i32{
    for i in input {
        let candidate1:i32 = *i;
        for j in input {
            let candidate2:i32 = *j;
            for k in input {
                let candidate3: i32 = *k;
                if candidate1 + candidate2 + candidate3 == sum_to_find {
                    println!("solution part 2: {}", candidate1 * candidate2 * candidate3);
                    return candidate1 * candidate2 * candidate3;
                }
            }
        }
    }
    return -1;
}
#[cfg(test)]
mod tests{
    use std::fs::File;
    use std::io::{BufReader, Read, BufRead};
    use crate::find_sum_and_multiply;
    use std::io;

    #[test]
    fn test_from_example()->io::Result<()>{//input and output given by the example output in the puzzle
        let file = File::open("src/test01.txt")?;
        let mut reader = BufReader::new(file);
        let lines = reader.by_ref().lines();
        let input_vec:Vec<i32> = lines.map(|i| i.ok().unwrap().parse().unwrap()).collect();

        let result = find_sum_and_multiply(&input_vec, 2020);
        let solution = 514579;
        assert_eq!(solution,result);
        Ok(())
    }

}
