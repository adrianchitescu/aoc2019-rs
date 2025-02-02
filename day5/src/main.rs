use std::env;
use std::fs;

extern crate int_computer;
use int_computer::computer::*;

fn parse_input(input: &str) -> Vec<i32> {
    let mut vec = Vec::new();
    for n in input.split_terminator(',') {
        if let Ok(f) = n.parse::<i32>() {
            vec.push(f);
        } else {
            eprintln!("invalid value in the provided input");
            break;
        }
    }
    vec
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args[1].clone();

    let file_contents = fs::read_to_string(&input_filename).unwrap_or_else(|err| {
        eprintln!("Error : {}", err);
        eprintln!("Cannot read from file {}", input_filename);
        std::process::exit(1);
    });

    let vec = parse_input(&file_contents);
    // Part1
    let mut comp = Computer::new32(&vec.clone());
    comp.add_input(1);
    comp.run();
    println!("Part1 answer: {:?}", comp.get_exit_value());

    // Part2
    let mut comp2 = Computer::new32(&vec.clone());
    comp2.add_input(5);
    comp2.run();
    println!("Part2 answer: {:?}", comp2.get_exit_value());
}
