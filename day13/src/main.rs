use std::fs;
use std::env;

use itertools::Itertools;
extern crate int_computer;
use int_computer::computer::*;


fn part1(instructions: &Vec<i128>) {
    let mut computer = Computer::new(&instructions);

    let output = computer.get_all_output();

    let block_tiles = output
        .chunks(3)
        .into_iter()
        .filter(|c| {
            c[2] == 2
        })
        .count();

    println!("blocks: {:?}", block_tiles);
}

fn part2(instructions: &Vec<i128>) {
    let mut computer = Computer::new(&instructions);

    computer.memwrite(0, 2);
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args[1].clone();

    let file_contents = fs::read_to_string(&input_filename).unwrap_or_else(|err| {
        eprintln!("Error : {}", err);
        eprintln!("Cannot read from file {}", input_filename);
        std::process::exit(1);
    });

    let instructions = int_computer::computer::read_instructions(&file_contents);

    part1(&instructions.clone());
}
