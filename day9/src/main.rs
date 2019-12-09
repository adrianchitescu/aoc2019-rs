use std::env;
use std::fs;

extern crate int_computer;
use int_computer::computer::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args[1].clone();

    let file_contents = fs::read_to_string(&input_filename).unwrap_or_else(|err| {
        eprintln!("Error : {}", err);
        eprintln!("Cannot read from file {}", input_filename);
        std::process::exit(1);
    });

    let mut computer = Computer::new(&int_computer::computer::read_instructions(&file_contents));
    computer.add_input(1);
    computer.run();

    println!("BOOST keycode = {}", computer.get_output().unwrap());

}
