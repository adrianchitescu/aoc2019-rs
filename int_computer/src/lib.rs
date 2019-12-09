pub mod computer {
    use std::collections::{HashMap, VecDeque};

    #[derive(PartialEq, Debug, Clone)]
    pub enum InstructionType {
        Add,
        Multiply,
        Input,
        Output,
        JumpIfTrue,
        JumpIfFalse,
        LessThan,
        Equals,
        AdjustBase,
        Exit,
    }

    #[derive(PartialEq)]
    pub enum State {
        WaitingInput,
        Done,
    }

    #[derive(Debug, Clone)]
    pub struct Instruction {
        itype: InstructionType,
        operands: Vec<usize>,
    }

    pub struct Computer {
        program: Vec<i128>,
        memory: HashMap<usize, i128>,
        output: VecDeque<i128>,
        input: VecDeque<i32>,
        instruction_pointer: usize,
        last_instr: Option<Instruction>,
        relative_base: i32,
    }

    pub fn read_instructions(input: &str) -> Vec<i128> {
        let mut vec = Vec::new();
        for n in input.split_terminator(',') {
            if let Ok(f) = n.parse::<i128>() {
                vec.push(f);
            } else {
                eprintln!("{}", n);
                eprintln!("invalid value in the provided input");
                break;
            }
        }
        vec
    }

    impl Computer {
        pub fn new(p: &Vec<i128>) -> Computer {
            let c = Computer {
                program: p.clone(),
                memory: HashMap::new(),
                output: VecDeque::new(),
                input: VecDeque::new(),
                instruction_pointer: 0,
                last_instr: None,
                relative_base: 0,
            };
            c
        }

        pub fn add_input(&mut self, v: i32) {
            self.input.push_back(v);
        }

        pub fn get_output(&mut self) -> Option<i128> {
            self.output.pop_front()
        }

        pub fn get_exit_value(&mut self) -> Option<i128> {
            self.output.pop_back()
        }

        pub fn get_positions(&mut self, n: usize) -> Vec<usize> {
            let mut o = vec![0; n];
            let op_code: i128 = &self.program[self.instruction_pointer] / 100;
            let mut op = op_code;
            let mut i = 0;
            self.instruction_pointer += 1;
            while i < n {
                match op % 10 {
                    0 => o[i] = self.program[self.instruction_pointer] as usize,
                    1 => {
                        o[i] = self.instruction_pointer as usize;
                    }
                    2 => {
                        o[i] = self.relative_base as usize
                            + (self.program[self.instruction_pointer] as usize);
                    }
                    _ => unreachable!(),
                };

                self.instruction_pointer += 1;
                i += 1;
                op /= 10;
            }

            o
        }
        fn next_instruction(&mut self) -> Instruction {
            use InstructionType::*;
            let instr = std::mem::replace(&mut self.last_instr, None);
            if let Some(i) = instr {
                return i;
            }

            let operation = self.program[self.instruction_pointer] as usize;
            let mut instr: Instruction = Instruction {
                itype: InstructionType::Add,
                operands: vec![],
            };
            match operation % 100 {
                1 => {
                    instr.itype = Add;
                    instr.operands = self.get_positions(3);
                }
                2 => {
                    instr.itype = Multiply;
                    instr.operands = self.get_positions(3);
                }
                3 => {
                    instr.itype = Input;
                    instr.operands = self.get_positions(1);
                }
                4 => {
                    instr.itype = Output;
                    instr.operands = self.get_positions(1);
                }
                5 => {
                    instr.itype = JumpIfTrue;
                    instr.operands = self.get_positions(2);
                }
                6 => {
                    instr.itype = JumpIfFalse;
                    instr.operands = self.get_positions(2);
                }
                7 => {
                    instr.itype = LessThan;
                    instr.operands = self.get_positions(3);
                }
                8 => {
                    instr.itype = Equals;
                    instr.operands = self.get_positions(3);
                }
                9 => {
                    instr.itype = AdjustBase;
                    instr.operands = self.get_positions(1);
                }
                99 => {
                    instr.itype = Exit;
                }
                _ => unreachable!(),
            };

            instr
        }

        pub fn run(&mut self) -> State {
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
                    self.output
                        .push_back(self.program[instr.operands[0]].clone());
                    continue;
                }

                let dest: usize;
                match instr.itype {
                    Add => {
                        dest = instr.operands[2];
                        self.program[dest] =
                            &self.program[instr.operands[0]] + &self.program[instr.operands[1]];
                    }
                    Multiply => {
                        dest = instr.operands[2];
                        self.program[dest] =
                            &self.program[instr.operands[0]] * &self.program[instr.operands[1]];
                    }
                    Input => {
                        dest = instr.operands[0];
                        if let Some(i) = self.input.pop_front() {
                            self.program[dest] = i as i128;
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
                    AdjustBase => {
                        self.relative_base += self.program[instr.operands[0]] as i32;
                    }
                    _ => unreachable!(),
                };
            }

            state
        }
    }
}
