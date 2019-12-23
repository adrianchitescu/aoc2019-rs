extern crate int_computer;

use std::env;
use std::fs;
use int_computer::computer::*;
use std::collections::{ HashMap, HashSet };

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_contents = std::fs::read_to_string(&args[1].clone()).unwrap_or_else(|err| {
        eprintln!("Error : {}", err);
        eprintln!("Cannot read from file ");
        std::process::exit(1);
    });
    let instructions = read_instructions(&file_contents);
    println!("{:?}", args);
    let mut cnt = 0;
    for y in 0..50 {
        for x in 0..50 {
            let mut computer: Computer = Computer::new(&instructions.clone());
            computer.add_input(x);
            computer.add_input(y);
            computer.run();
            if let Some(pulled) = computer.get_output() {
                if pulled == 1 {
                    print!("#");
                    cnt += 1;
                } else {
                    print!(".");
                }
            } else {
                println!("no output");
            }
        }
        println!();
    }
    println!("total in 50x50 = {}", cnt);
}
