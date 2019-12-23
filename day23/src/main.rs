extern crate int_computer;

use std::env;
use std::fs;
use int_computer::computer::*;
use std::collections::{ HashMap, HashSet };

struct Network {
    computers: Vec<Computer>,
}

impl Network {
    fn new(instructions: &Vec<i128>) -> Network {
        let mut n = Network {
            computers: Vec::new()
        };
        for i in 0..50 {
            let mut c = Computer::new(&instructions.clone());
            c.add_input(i as i32);
            // c.run();
            n.computers.push(c);
        }
        n
    }

    fn run(&mut self) {
        let mut nat = (0,0);
        let mut idle = false;
        let mut nat_values = HashSet::new();
        let mut answers : HashMap<&str, i128> = HashMap::new();
        while answers.len() < 2 {
            if idle {
                if nat_values.insert(nat.1) {
                    self.computers[0].add_input_128(nat.0);
                    self.computers[0].add_input_128(nat.1);
                } else {
                    answers.entry("part2").or_insert(nat.1);
                }
            }
            idle = true;
            for i in 0..50 {
                if !self.computers[i].has_input() {
                    self.computers[i].add_input(-1);
                }
                self.computers[i].run();
                let out = self.computers[i].get_all_output();
                for packet in out.chunks(3) {
                    if packet.len() != 3 {
                        println!("Invalid packet size {} {:?}", packet.len(), packet);
                        break;
                    } else if packet[0] == 255 {
                        answers.entry("part1").or_insert(packet[2]);
                        nat = (packet[1], packet[2]);
                    } else if packet[0] >= 0 && packet[0] < 50 {
                        self.computers[packet[0] as usize].add_input_128(packet[1]);
                        self.computers[packet[0] as usize].add_input_128(packet[2]);
                    } else {
                        println!("oups");
                        break;
                    }
                    idle = false;
                }
            }
        }

        println!("{:?}", answers)
    }
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
    let mut net = Network::new(&instructions);
    net.run();
}
