use std::env;
use std::fs;
use crate::InstructionType::{Exit, Output};

#[derive(PartialEq, Debug)]
enum InstructionType {
    Add,
    Multiply,
    Input,
    Output,
    Exit,
}
#[derive(Debug)]
struct Instruction<'a> {
    itype: InstructionType,
    operands : Vec<usize>,
    program: &'a Vec<i32>
}

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
fn get_positions(p:&Vec<i32>, ip: &mut usize, n: usize) -> Vec<usize> {
    let mut o = vec![0; n];
    let mut op = p[*ip as usize] / 100;
    let mut i = 0;
    *ip += 1;
    while i < n {
       if op % 10 == 0 {
          o[i] = p[*ip as usize] as usize;
       } else {
          o[i] = *ip as usize;
       }
       *ip += 1;
        i += 1;
        op /= 10;
    }

    o
}
fn get_instruction<'a>(p: &'a Vec<i32>, ip: &mut usize) -> Instruction<'a> {
    use InstructionType::*;
    let operation = p[*ip];
    let mut instr: Instruction = Instruction { itype: InstructionType::Add, operands: vec![], program : p };
    match operation % 100 {
        1 => { instr.itype = Add; instr.operands = get_positions(p, ip, 3);}
        2 => { instr.itype = Multiply; instr.operands = get_positions(p, ip, 3);}
        3 => { instr.itype = Input; instr.operands = get_positions(p, ip, 1);}
        4 => { instr.itype = Output; instr.operands = get_positions(p, ip, 1);}
        99 =>{ instr.itype = Exit;}
        _ => unreachable!()
    };

    instr
}

fn part1(v: &Vec<i32>) -> i32 {
    let mut pos = 0;
    let mut vec = v.to_vec();
    loop {
        let instr = get_instruction(&vec, &mut pos);

        if instr.itype == Exit {
            break;
        }
        if instr.itype == Output {
            continue;
        }

        let dest: usize;
        let result = match instr.itype {
            InstructionType::Add=> {
                dest = instr.operands[2];
                vec[instr.operands[0]] + vec[instr.operands[1]]
            }
            InstructionType::Multiply=> {
                dest = instr.operands[2];
                vec[instr.operands[0]] * vec[instr.operands[1]]
            }
            InstructionType::Input=> {
                dest = instr.operands[0];
                1
            }

            _ => unreachable!()
        };

        vec[dest] = result;
        println!("[{}] = {}",dest, result);
    }

    vec[0]
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args[1].clone();

    let file_contents = fs::read_to_string(&input_filename)
        .unwrap_or_else(|err| {
            eprintln!("Error : {}", err);
            eprintln!("Cannot read from file {}", input_filename);
            std::process::exit(1);
        });

    let vec = parse_input(&file_contents);
    part1(&vec);
}
