use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;
use std::ops::Index;

fn main() {
    println!("result day 1: {:?}", process_instructions(&parse_instructions(read_input("src/input.txt"))));
    println!("result day 2: {:?}", modify_instructions(parse_instructions(read_input("src/input.txt"))));
}

fn parse_instructions(lines: Vec<String>) -> Vec<(String, i32)> {
    let mut instructions: Vec<(String, i32)> = Vec::new();
    for line in lines {
        let parsed: Vec<&str> = line.split(' ').collect();
        let op = Some(parsed.get(0).clone()).unwrap();
        let arg = Some(parsed.get(1).clone()).unwrap();
        //println!("op:{:?}, arg:{:?}",op.unwrap(),i32::from_str(arg.unwrap()).unwrap());
        instructions.push((op.unwrap().parse().unwrap(), i32::from_str(arg.unwrap()).unwrap()));
    }
    return instructions;
}

fn modify_instructions(mut instructions: Vec<(String, i32)>) {
    for (i,mut instruction) in instructions.first_mut().unwrap().enumerate() {
        //change operation
        if instruction.0 == "nop" {
            instruction[i].0 = "jmp".to_string();
        } else if instruction.0 == "jmp" {
            instructions[i].0 = "nop".to_string();
        }
        //execute program
        let (acc, is_terminated) = process_instructions(&instructions);
        println!("{:?}", (acc,is_terminated));
        if is_terminated {
            println!("acc after termination {}", acc);
            return;
        }
        //change back operation
        if instruction.0 == "nop" {
            instruction[i].0 = "jmp".to_string();
        } else if instruction.0 == "jmp" {
            instructions[i].0 = "nop".to_string();
        }
    }
}

fn process_instructions(instructions: &Vec<(String, i32)>) -> (i32, bool) {
    let mut executed_instructions: HashSet<usize> = HashSet::new();
    let mut ip: usize = 0;
    let mut acc: i32 = 0;
    let mut current_instruction = instructions.get(ip).unwrap();
    while !executed_instructions.contains(&ip) { //we halt if we would execute an instruction twice
        executed_instructions.insert(ip);
        //println!("ip:{}, acc:{}", ip, acc);
        match current_instruction.0.as_str() {
            "nop" => ip += 1,
            "acc" => { ip += 1; acc += current_instruction.1 },
            "jmp" => ip = (ip as i32 + current_instruction.1) as usize,
            _ => unreachable!()
        }
        //println!("instruction:{:?}", current_instruction);
        if ip == instructions.len() - 1 {
            return (acc, true);
        }
        current_instruction = instructions.get(ip).unwrap();
    }
    return (acc, ip == instructions.len() - 1);
}

fn read_input(filename_path: &str) -> Vec<String> {
    let file = File::open(filename_path).unwrap();
    let mut reader = BufReader::new(file);
    let lines = reader.by_ref().lines();
    let input_vec: Vec<String> = lines.map(|l| l.ok().unwrap()).collect();
    return input_vec;
}
