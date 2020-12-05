use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

fn main() -> io::Result<()> {
    let input = read_input("src/input.txt");
    let input2 = read_input("src/input.txt");
    println!("solution part1: {}", find_highest_seat_id(input));
    println!("solution part2: {:?}", find_missing_seat(get_seatid_list(input2)));
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

fn get_seatid_list(lines: Vec<String>) -> Vec<i32> {
    let mut seatids: Vec<i32> = vec![];
    for line in lines {
        let seat: (i32, i32) = get_seat(line);
//        println!("{:?}",seat);
        seatids.push(seat.0 * 8 + seat.1);
    }
    seatids.sort();
    println!("{:?}", seatids);
    return seatids;
}

fn find_missing_seat(seatids: Vec<i32>) -> i32 {
    for i in 0..seatids.len() - 1 {
        if seatids[i] + 1 != seatids[i + 1] {
            return seatids[i] + 1;
        }
    }
    return 0;
}

fn find_highest_seat_id(lines: Vec<String>) -> i32 {
    let mut highest_seat_id = 0;
    for line in lines {
        let seat: (i32, i32) = get_seat(line);
//        println!("found seat {:?}",seat);
        let seat_id = seat.0 * 8 + seat.1;
        if seat_id > highest_seat_id {
            highest_seat_id = seat_id
        }
    }
    return highest_seat_id;
}

fn get_seat(indications: String) -> (i32, i32) {
    let rows = (0..128).collect();
    let cols = (0..8).collect();
    let row_indications = &indications[0..7];
    let column_indications = &indications[7..10];
    println!("using row indications {}",row_indications);
    println!("using col indications {}",column_indications);
    return (find_number(row_indications.chars().collect(), rows),
            find_number(column_indications.chars().collect(), cols));
}


fn find_number(instructions: Vec<char>, seats: Vec<i32>) -> i32 {
    let mut remaining_seats = seats;
    println!("instructions are {:?}", instructions);
    // println!("seats are {:?}",remaining_seats);
    for instruction in &instructions[..instructions.len()] {
        remaining_seats = match instruction {
            'F' => remaining_seats.drain(..(remaining_seats.len() / 2)).collect(),
            'B' => remaining_seats.drain(remaining_seats.len() / 2..).collect(),
            'L' => remaining_seats.drain(..(remaining_seats.len() / 2)).collect(),
            'R' => remaining_seats.drain(remaining_seats.len() / 2..).collect(),
            _ => { vec![-1] }
        };
        println!("range {:?},{:?}", remaining_seats.first().unwrap(), remaining_seats.last().unwrap());
        if remaining_seats.contains(&-1) {
            println!("this should never happen!");
        }
    }
//    println!("size after instr {}",remaining_seats.len());
    let last_instr = instructions.last().unwrap();
    println!("last instr {}", last_instr);
    let number = match last_instr {
        'F' => *remaining_seats.first().unwrap(),
        'B' => *remaining_seats.last().unwrap(),
        'L' => *remaining_seats.first().unwrap(),
        'R' => *remaining_seats.last().unwrap(),
        _ => { -1 }
    };

    println!("row/col {}", number);
    return number;
//return    *remaining_seats.first().unwrap()
}


//sadly this recursive variant generates a stackoverflow
/*fn find_number(mut instructions: Vec<char>, seats: Vec<i32>) -> i32 {
    if (!instructions.len() > 1) {
            return find_number(instructions, seats);
    } else {
        let instruction = instructions.pop().unwrap();
        if instruction == 'F' || instruction == 'L' {
            return *seats.first().unwrap();
        } else {
            return *seats.last().unwrap();
        }
        return 0;
    }
}*/


#[cfg(test)]
mod tests {
    use std::io;

    use crate::{find_highest_seat_id, find_number, get_seat, get_seatid_list, read_input};

    #[test]
    fn test_from_example() -> io::Result<()> {//input and output given by the example output in the puzzle
        //BFFFBBFRRR: row 70, column 7, seat ID 567.
        //FFFBBBFRRR: row 14, column 7, seat ID 119.
        //BBFFBBFRLL: row 102, column 4, seat ID 820.
        let solution = 820;
        assert_eq!(solution, find_highest_seat_id(read_input("src/test.txt")));
        Ok(())
    }

    #[test]
    fn test_drain() -> io::Result<()> {//input and output given by the example output in the puzzle
        let mut rows: Vec<i32> = (0..128).collect();
//        println!("{:?}",rows.drain(..rows.len()/2));
//        println!("{:?}",rows);
        rows = rows.drain(rows.len() / 2..).collect();
        println!("{:?}", rows);
        Ok(())
    }

    #[test]
    fn test_find_number_from_example() -> io::Result<()> {//input and output given by the example output in the puzzle
        let mut rows: Vec<i32> = (0..128).collect();
        let mut cols: Vec<i32> = (0..8).collect();
//        println!("{:?}",rows.drain(..rows.len()/2));
//        println!("{:?}",rows);
        let instruction = "FBFBBFF".chars().collect();
        assert_eq!(44, find_number(instruction, rows));
        let instruction = "RLR".chars().collect();
        assert_eq!(5, find_number(instruction, cols));
        Ok(())
    }

    #[test]
    fn test_find_number_from_testfile() -> io::Result<()> {//input and output given by the example output in the puzzle
        let mut rows: Vec<i32> = (0..128).collect();
        let mut cols: Vec<i32> = (0..8).collect();
//        println!("{:?}",rows.drain(..rows.len()/2));
//        println!("{:?}",rows);
        let instruction = "BFFFBBF".chars().collect();
        assert_eq!(70, find_number(instruction, rows));
        let instruction = "RRR".chars().collect();
        assert_eq!(7, find_number(instruction, cols));
        Ok(())
    }

    #[test]
    fn test_verify_get_seat() -> io::Result<()> {
        assert_eq!((70, 7), get_seat("BFFFBBFRRR".chars().collect()));
        assert_eq!((14, 7), get_seat("FFFBBBFRRR".chars().collect()));
        assert_eq!((102, 4), get_seat("BBFFBBFRLL".chars().collect()));
        Ok(())
    }
}
