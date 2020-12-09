use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use daggy::{Dag, NodeIndex};
use regex::Regex;

fn main() {
    parse_input(read_input("src/input.txt"));
}

fn read_input(filename_path: &str) -> Vec<String> {
    let file = File::open(filename_path).unwrap();
    let mut reader = BufReader::new(file);
    let lines = reader.by_ref().lines();
    let input_vec: Vec<String> = lines.map(|l| l.ok().unwrap()).collect();
    return input_vec;
}

struct Weight;

fn parse_input(input: Vec<String>) -> (Dag<Weight, u32, u32>, NodeIndex<u32>) {
    let mut dag = Dag::<Weight, u32, u32>::new();
    let root = dag.add_node(Weight);

    let re = Regex::new(r"^(\w+ \w+)|(\d+) (\w+ \w+) bag").unwrap();

    for line in input {
        println!("line {}", line);
        for cap in re.captures_iter(&line) {
            // println!("cap {:?}", cap);
            let mut containing_bag: &str;
            let mut bags:Vec<(i32,&str)> = Vec::new();
            if cap.get(1).is_some() {//containing bag
                containing_bag=&cap[1];
                println!("containing bag: {}",&cap[1]);
            }
            else if cap.get(2).is_some(){//contained bags
                bags.push(((&cap[2]).parse().unwrap(), &cap[3]));
                println!("found bag's {:?} {:?}", &cap[2],&cap[3]);
            }
        }
    }
    return (dag, root);
}
