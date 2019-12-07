

pub mod computer {
    use std::collections::VecDeque;

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
        program: Vec<i32>,
        output: VecDeque<i32>,
        input: VecDeque<i32>,
        instruction_pointer: usize,
        last_instr: Option<Instruction>,
    }

    impl Computer {
        pub fn new(p: &Vec<i32>) -> Computer {
            Computer {
                program: p.clone(),
                output: VecDeque::new(),
                input: VecDeque::new(),
                instruction_pointer: 0,
                last_instr: None,
            }
        }

        pub fn add_input(&mut self, v: i32) {
            self.input.push_back(v);
        }

        pub fn get_output(&mut self) -> Option<i32> {
            self.output.pop_front()
        }
        pub fn get_exit_value(&mut self) -> Option<i32> {
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
}
