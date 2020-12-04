use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::time::Instant;

fn main() -> io::Result<()> {
    let now = Instant::now();
    let input = read_input("src/day03.txt");
    println!("solution part1: {}", count_encountered_trees(input, (1, 3)));
    println!("part 1 took {}ms",now.elapsed().as_millis());




    let now2 = Instant::now();
    let slopes: Vec<(i32, i32)> = vec!((1, 1), (1, 3), (1, 5), (1, 7), (2, 1));
    let mut slope_counts:Vec<i32> = vec![];
    for slope in slopes {
        let input2 = read_input("src/day03.txt");
        slope_counts.push(count_encountered_trees(input2, slope));
    }
    println!("solution part2:{:?}", slope_counts);
    println!("part 2 took {}ms",now2.elapsed().as_millis());
    println!("execution finished");

    Ok(())
}

fn read_input(filename_path: &str) -> Vec<Vec<char>> {
    let file = File::open(filename_path).unwrap();
    let mut reader = BufReader::new(file);
    let lines = reader.by_ref().lines();
    let input_vec: Vec<Vec<char>> = lines.map(|l| l.ok().unwrap().chars().collect()).collect();
    return input_vec;
}


fn count_encountered_trees(input: Vec<Vec<char>>, slope: (i32, i32)) -> i32 {
    let tree: char = '#';
    //let empty: char = '.';
    let width: i32 = input[0].len() as i32;
    let height: i32 = input.len() as i32;
    let mut xpos: i32 = 0;
    let mut ypos: i32 = 0;
    let mut num_encountered_trees = 0;
//    println!("height: {}", height);
//    println!("width: {}", width);
    while ypos < height {
        //println!("accessing [{}][{}]", ypos, xpos);
        if input[ypos as usize][xpos as usize] == tree {
            num_encountered_trees += 1;
        }
        ypos += slope.0;
        xpos = (xpos + slope.1) % width; //forest ist copied to the right indefinitely so we just wrap around
    }
    return num_encountered_trees;
}

#[cfg(test)]
mod tests {
    use std::io;

    use crate::{count_encountered_trees, read_input};

    #[test]
    fn test_from_example() -> io::Result<()> {//input and output given by the example output in the puzzle
        let solution = 7;
        assert_eq!(solution, count_encountered_trees(read_input("src/test.txt"), (1, 3)));
        Ok(())
    }
}
