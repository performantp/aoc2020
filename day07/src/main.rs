use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use daggy::NodeIndex;
use daggy::petgraph::{Directed, Graph};
use regex::Regex;

fn main() {
    let graph1 = parse_input(read_input("src/input.txt"));
    println!("walked over {} nodes",visit_nodes_recursively(graph1.0,graph1.1));
}

fn read_input(filename_path: &str) -> Vec<String> {
    let file = File::open(filename_path).unwrap();
    let mut reader = BufReader::new(file);
    let lines = reader.by_ref().lines();
    let input_vec: Vec<String> = lines.map(|l| l.ok().unwrap()).collect();
    return input_vec;
}

fn parse_input(input: Vec<String>) -> (Graph<String, i32, Directed, u32>,NodeIndex) {
    let mut graph = Graph::<String, i32>::new();
    let re = Regex::new(r"^(\w+ \w+)|(\d+) (\w+ \w+) bag").unwrap();
    let mut containing_bag = String::new().clone();
    let mut start: NodeIndex = Default::default();
    let mut containing_bag_node = NodeIndex::new(1);
    let mut node_directory:HashMap<String,NodeIndex> = HashMap::new();
    for line in input.into_iter() {
        for cap in re.captures_iter(&line) {
            //containing bag
            //println!("node directory: {:?}",node_directory);
            if cap.get(1).is_some() {
                containing_bag = String::from(cap.get(1).unwrap().as_str());
                containing_bag_node = *node_directory.entry(containing_bag.as_str().to_string()).or_insert(graph.add_node((&containing_bag).to_string()));
              //  println!("found bag with name {} and put it to {:?}",&containing_bag,&containing_bag_node);
            }
            //contained bags
            else if cap.get(2).is_some() {
                let bag_name = String::from(cap.get(3).unwrap().as_str());
                let number_of_bags: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
                //add to graph
                let contained_bag_node = node_directory.entry((&bag_name).to_string()).or_insert(graph.add_node((&bag_name).to_string()));
               // println!("found bag with name {} and put it to {:?}",bag_name,contained_bag_node);
                if bag_name == "shiny gold" { start = containing_bag_node; }
                graph.update_edge(*contained_bag_node, containing_bag_node, number_of_bags);
            }
        }
        println!("node directory: {:?}",node_directory);
    }
    println!("nodes {:?}", graph.node_count());
    println!("edges {:?}", graph.edge_count());
    println!("graph {:?}", graph);
    return (graph,start);
}

fn visit_nodes_recursively(graph:Graph<String, i32>, node:NodeIndex) ->i32{
    println!("visiting node{:?}",node);
    if graph.neighbors(node).enumerate().count()==0 {return 1;}
    let mut accum = 0;
    for x in graph.neighbors(node).into_iter() {
        accum+=visit_nodes_recursively(graph.clone(),x);
    }
    accum+=1;//add one for current node
    return accum;

}