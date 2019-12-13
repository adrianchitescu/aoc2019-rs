use std::env;
use std::fs;

extern crate int_computer;
use int_computer::computer::*;

fn part1(computer: &mut Computer) -> usize {
    computer.run();
    let output = computer.get_all_output();
    let block_tiles = output.chunks(3).into_iter().filter(|c| c[2] == 2).count();

    println!("block_tiles {}", block_tiles);
    block_tiles
}

fn part2(instructions: &Vec<i128>) {
    let mut computer = Computer::new(&instructions);
    let mut pad_x = 0;
    let mut ball_x = 0;
    let mut total_score = 0;

    computer.memwrite(0, 2);

    loop {
        let state = computer.run();
        let output = computer.get_all_output();
        output.chunks(3).into_iter().for_each(|c| {
            if c[0] == -1 {
                total_score = c[2] as i32;
            }
            match c[2] {
                3 => pad_x = c[0] as i32,
                4 => ball_x = c[0] as i32,
                _ => {}
            }
        });

        if state == State::Done {
            break;
        }
        computer.add_input((ball_x - pad_x).signum());
    }
    println!("Score : {}", total_score);
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

    part1(&mut Computer::new(&instructions.clone()));
    part2(&instructions);
}
