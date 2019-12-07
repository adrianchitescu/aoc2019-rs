use permutator::Permutation;
use std::collections::VecDeque;
use std::env;
use std::fs;

#[derive(PartialEq, Debug, Clone)]
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
#[derive(PartialEq)]
enum State {
    WaitingInput,
    Done,
}

#[derive(Debug, Clone)]
struct Instruction {
    itype: InstructionType,
    operands: Vec<usize>,
}
struct Computer {
    program: Vec<i32>,
    output: VecDeque<i32>,
    input: VecDeque<i32>,
    instruction_pointer: usize,
    last_instr: Option<Instruction>,
}

impl Computer {
    fn new(p: &Vec<i32>) -> Computer {
        Computer {
            program: p.clone(),
            output: VecDeque::new(),
            input: VecDeque::new(),
            instruction_pointer: 0,
            last_instr: None,
        }
    }

    fn add_input(&mut self, v: i32) {
        self.input.push_back(v);
    }

    fn get_output(&mut self) -> Option<i32> {
        self.output.pop_front()
    }
    fn get_exit_value(&mut self) -> Option<i32> {
        self.output.pop_back()
    }
    fn next_instruction(&mut self) -> Instruction {
        use InstructionType::*;
        let instr = std::mem::replace(&mut self.last_instr, None);
        if let Some(i) = instr {
            return i;
        }

        let operation = self.program[self.instruction_pointer];
        let mut instr: Instruction = Instruction {
            itype: InstructionType::Add,
            operands: vec![],
        };
        match operation % 100 {
            1 => {
                instr.itype = Add;
                instr.operands = get_positions(&self.program, &mut self.instruction_pointer, 3);
            }
            2 => {
                instr.itype = Multiply;
                instr.operands = get_positions(&self.program, &mut self.instruction_pointer, 3);
            }
            3 => {
                instr.itype = Input;
                instr.operands = get_positions(&self.program, &mut self.instruction_pointer, 1);
            }
            4 => {
                instr.itype = Output;
                instr.operands = get_positions(&self.program, &mut self.instruction_pointer, 1);
            }
            5 => {
                instr.itype = JumpIfTrue;
                instr.operands = get_positions(&self.program, &mut self.instruction_pointer, 2);
            }
            6 => {
                instr.itype = JumpIfFalse;
                instr.operands = get_positions(&self.program, &mut self.instruction_pointer, 2);
            }
            7 => {
                instr.itype = LessThan;
                instr.operands = get_positions(&self.program, &mut self.instruction_pointer, 3);
            }
            8 => {
                instr.itype = Equals;
                instr.operands = get_positions(&self.program, &mut self.instruction_pointer, 3);
            }
            99 => {
                instr.itype = Exit;
            }
            _ => unreachable!(),
        };

        instr
    }
    fn run(&mut self) -> State {
        use InstructionType::*;
        let state: State;
        loop {
            if self.instruction_pointer >= self.program.len() {
                state = State::Done;
                break;
            }
            let instr = self.next_instruction();

            if instr.itype == Exit {
                state = State::Done;
                break;
            }
            if instr.itype == Output {
                self.output.push_back(self.program[instr.operands[0]]);
                continue;
            }

            let dest: usize;
            match instr.itype {
                Add => {
                    dest = instr.operands[2];
                    self.program[dest] =
                        self.program[instr.operands[0]] + self.program[instr.operands[1]];
                }
                Multiply => {
                    dest = instr.operands[2];
                    self.program[dest] =
                        self.program[instr.operands[0]] * self.program[instr.operands[1]];
                }
                Input => {
                    dest = instr.operands[0];
                    if let Some(i) = self.input.pop_front() {
                        self.program[dest] = i;
                    } else {
                        self.last_instr = Some(instr);
                        state = State::WaitingInput;
                        break;
                    }
                }
                JumpIfTrue => {
                    if self.program[instr.operands[0]] != 0 {
                        self.instruction_pointer = self.program[instr.operands[1]] as usize;
                    }
                }
                JumpIfFalse => {
                    if self.program[instr.operands[0]] == 0 {
                        self.instruction_pointer = self.program[instr.operands[1]] as usize;
                    }
                }
                LessThan => {
                    if self.program[instr.operands[0]] < self.program[instr.operands[1]] {
                        self.program[instr.operands[2]] = 1;
                    } else {
                        self.program[instr.operands[2]] = 0;
                    }
                }
                Equals => {
                    if self.program[instr.operands[0]] == self.program[instr.operands[1]] {
                        self.program[instr.operands[2]] = 1;
                    } else {
                        self.program[instr.operands[2]] = 0;
                    }
                }

                _ => unreachable!(),
            };
        }

        state
    }
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

fn get_positions(p: &Vec<i32>, ip: &mut usize, n: usize) -> Vec<usize> {
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

fn run_amplifiers(phase_seq: &Vec<i32>, program: &Vec<i32>) -> i32 {
    let mut amplifiers: Vec<Computer> = phase_seq
        .into_iter()
        .map(|phase| {
            let mut c = Computer::new(&program);
            c.add_input(*phase);
            c
        })
        .collect();

    let mut last_out = Some(0);
    let mut index = 0;
    loop {
        if let Some(o) = last_out {
            amplifiers[index].add_input(o);
        }
        let s = amplifiers[index].run();
        if s == State::Done && index == amplifiers.len() - 1 {
            last_out = amplifiers[index].get_exit_value();
            break;
        }

        last_out = amplifiers[index].get_output();
        index = (index + 1) % amplifiers.len();
    }

    last_out.unwrap()
}

fn get_max_signal(program: &Vec<i32>, phase: &[i32]) -> i32 {
    if let Some(max_signal) = phase.to_vec()
        .permutation()
        .into_iter()
        .map(|p| run_amplifiers(&p, &program.clone()))
        .max_by(|x, y| x.cmp(y))
    {
        println!("Max signal is {}", max_signal);
        max_signal
    } else {
        println!("Max signal is missing");
        -1
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

    let vec = parse_input(&file_contents);
    get_max_signal(&vec.clone(), &[0, 1, 2, 3, 4]);
    get_max_signal(&vec.clone(), &[5, 6, 7, 8, 9]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amp() {
        let vec = parse_input("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        assert_eq!(get_max_signal(&vec.clone(), &[0, 1, 2, 3, 4]), 43210);
    }

    #[test]
    fn test_amp2() {
        let vec =
            parse_input("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
        assert_eq!(get_max_signal(&vec.clone(), &[0, 1, 2, 3, 4]), 54312);
    }

    #[test]
    fn test_amp3() {
        let vec = parse_input(
            "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
        );
        assert_eq!(get_max_signal(&vec.clone(), &[0, 1, 2, 3, 4]), 65210);
    }

    #[test]
    fn test_loop_amp() {
        let vec = parse_input(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        );
        assert_eq!(get_max_signal(&vec.clone(), &[5, 6, 7, 8, 9]), 139629729);
    }

    #[test]
    fn test_loop_amp2() {
        let vec = parse_input(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
        );
        assert_eq!(get_max_signal(&vec.clone(), &[5, 6, 7, 8, 9]), 18216);
    }
}
