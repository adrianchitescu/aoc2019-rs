extern crate int_computer;

use std::env;
use int_computer::computer::*;

fn display(computer: &mut Computer) {
    computer.run();
    let last = computer.get_exit_value();
    let output: Vec<i128> = computer.get_all_output();

    for c in output {
        print!("{}", (c as u8) as char);
    }
    println!("{:?}", last);
}

fn run(computer: &mut Computer, script: &Vec<&str>) {
    script
        .iter()
        .for_each(|s| {
            for c in s.chars() {
                computer.add_input(c as i32);
            }
            computer.add_input(10);
        });

    display(computer);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut computer = Computer::new_from_file(&args[1]);
    let mut computer2 = Computer::new_from_file(&args[1]);

    //    J = !(A & B & C) & D
    run(&mut computer, &vec![
        "OR A J",
        "AND B J",
        "AND C J",
        "NOT J J",
        "AND D J",
        "WALK"
    ]);

    //    JUMP if part1 conditions are true and E is solid or H is solid so the next jump is valid
    //    J = !(A & B & C) & D & (E | H)
    run(&mut computer2, &vec![
        "OR A J",
        "AND B J",
        "AND C J",
        "NOT J J",
        "AND D J",
        "OR E T",
        "OR H T",
        "AND T J",
        "RUN"
    ]);
}
