use std::env;
use std::fs;

#[derive(PartialEq, Debug)]
enum InstructionType {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Exit,
}
#[derive(Debug)]
struct Instruction {
    itype: InstructionType,
    operands : Vec<usize>,
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
fn get_instruction(p: & Vec<i32>, ip: &mut usize) -> Instruction {
    use InstructionType::*;
    let operation = p[*ip];
    let mut instr: Instruction = Instruction { itype: InstructionType::Add, operands: vec![]};
    match operation % 100 {
        1 => { instr.itype = Add;           instr.operands = get_positions(p, ip, 3);}
        2 => { instr.itype = Multiply;      instr.operands = get_positions(p, ip, 3);}
        3 => { instr.itype = Input;         instr.operands = get_positions(p, ip, 1);}
        4 => { instr.itype = Output;        instr.operands = get_positions(p, ip, 1);}
        5 => { instr.itype = JumpIfTrue;    instr.operands = get_positions(p, ip, 2);}
        6 => { instr.itype = JumpIfFalse;   instr.operands = get_positions(p, ip, 2);}
        7 => { instr.itype = LessThan;      instr.operands = get_positions(p, ip, 3);}
        8 => { instr.itype = Equals;        instr.operands = get_positions(p, ip, 3);}
        99 =>{ instr.itype = Exit;}
        _ => unreachable!()
    };

    instr
}

fn run(v: &Vec<i32>, input: i32) -> i32 {
    use InstructionType::*;
    let mut pos = 0;
    let mut vec = v.to_vec();
    let mut output: i32 = 0;
    loop {
        let instr = get_instruction(&vec, &mut pos);

        if instr.itype == Exit {
            break;
        }
        if instr.itype == Output {
            output = vec[instr.operands[0]];
            continue;
        }

        let dest: usize;
        match instr.itype {
            Add => {
                dest = instr.operands[2];
                vec[dest] = vec[instr.operands[0]] + vec[instr.operands[1]];
            }
            Multiply => {
                dest = instr.operands[2];
                vec[dest] = vec[instr.operands[0]] * vec[instr.operands[1]];
            }
            Input => {
                dest = instr.operands[0];
                vec[dest] = input;
            }
            JumpIfTrue  => {
                if vec[instr.operands[0]] != 0 {
                    pos = vec[instr.operands[1]] as usize;
                }
            }
            JumpIfFalse => {
                if vec[instr.operands[0]] == 0 {
                    pos = vec[instr.operands[1]] as usize;
                }
            }
            LessThan => {
                if vec[instr.operands[0]] < vec[instr.operands[1]] {
                    vec[instr.operands[2]] = 1;
                } else {
                    vec[instr.operands[2]] = 0;
                }
            }
            Equals => {
                if vec[instr.operands[0]] == vec[instr.operands[1]] {
                    vec[instr.operands[2]] = 1;
                } else {
                    vec[instr.operands[2]] = 0;
                }
            }

            _ => unreachable!()
        };
    }

    output
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

    let vec1 = parse_input(&file_contents);
    let vec2 = vec1.clone();
    println!("Part1 answer: {}", run(&vec1, 1));
    println!("Part2 answer: {}", run(&vec2, 5));
}
