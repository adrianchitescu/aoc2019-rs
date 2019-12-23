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

    #[derive(PartialEq, Debug)]
    pub enum State {
        WaitingInput,
        Done,
    }

    #[derive(Debug, Clone)]
    pub struct Instruction {
        itype: InstructionType,
        operands: Vec<i128>,
    }

    pub struct Computer {
        memory: HashMap<i128, i128>,
        output: VecDeque<i128>,
        input: VecDeque<i128>,
        instruction_pointer: i128,
        last_instr: Option<Instruction>,
        relative_base: i128,
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
            let mut c = Computer {
                memory: HashMap::new(),
                output: VecDeque::new(),
                input: VecDeque::new(),
                instruction_pointer: 0,
                last_instr: None,
                relative_base: 0,
            };

            c.memory = (0..)
                .into_iter()
                .zip(p.into_iter().cloned())
                .into_iter()
                .collect();

            c
        }
        pub fn new32(p: &Vec<i32>) -> Computer {
            Computer::new(&p.into_iter().cloned().map(|x| x as i128).collect())
        }

        pub fn new_from_file(filename: &String) -> Computer {
            let file_contents = std::fs::read_to_string(&filename).unwrap_or_else(|err| {
                eprintln!("Error : {}", err);
                eprintln!("Cannot read from file {}", filename);
                std::process::exit(1);
            });

            Computer::new( &read_instructions(&file_contents))
        }

        pub fn memwrite(&mut self, pos: i128, value: i128) {
            self.memory.insert(pos, value);
        }

        fn memread(&self, pos: i128) -> i128 {
            if pos < 0 {
                unreachable!();
            }
            match self.memory.get(&pos) {
                Some(v) => *v,
                None => 0,
            }
        }
        fn is_valid_mem(&self, pos:i128) -> bool {
            match self.memory.get(&pos) {
                Some(_) => true,
                None => false,
            }
        }

        pub fn has_input(&self) -> bool {
            !self.input.is_empty()
        }

        pub fn add_input(&mut self, v: i32) {
            self.input.push_back(v as i128);
        }

        pub fn add_input_128(&mut self, v: i128) {
            self.input.push_back(v);
        }

        pub fn get_output(&mut self) -> Option<i128> {
            self.output.pop_front()
        }

        pub fn get_all_output(&mut self) -> Vec<i128> {
            self.output.drain(..).collect()
        }

        pub fn get_exit_value(&mut self) -> Option<i128> {
            self.output.pop_back()
        }

        pub fn get_positions(&mut self, n: usize) -> Vec<i128> {
            let mut o = vec![0; n];
            let mut op = self.memread(self.instruction_pointer) / 100;
            let mut i = 0;
            self.instruction_pointer += 1;
            while i < n {
                match op % 10 {
                    0 => o[i] = self.memread(self.instruction_pointer),
                    1 => {
                        o[i] = self.instruction_pointer;
                    }
                    2 => {
                        o[i] = self.relative_base + (self.memread(self.instruction_pointer));
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

            let operation = self.memread(self.instruction_pointer) as usize;
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
                if !self.is_valid_mem(self.instruction_pointer) {
                    state = State::Done;
                    break;
                }
                let instr = self.next_instruction();
                if instr.itype == Exit {
                    state = State::Done;
                    break;
                }
                if instr.itype == Output {
                    self.output.push_back(self.memread(instr.operands[0]));
                    continue;
                }

                match instr.itype {
                    Add => {
                        self.memwrite(
                            instr.operands[2],
                            self.memread(instr.operands[0]) + self.memread(instr.operands[1]),
                        );
                    }
                    Multiply => {
                        self.memwrite(
                            instr.operands[2],
                            self.memread(instr.operands[0]) * self.memread(instr.operands[1]),
                        );
                    }
                    Input => {
                        if let Some(i) = self.input.pop_front() {
                            self.memwrite(instr.operands[0], i as i128);
                        } else {
                            self.last_instr = Some(instr);
                            state = State::WaitingInput;
                            break;
                        }
                    }
                    JumpIfTrue => {
                        if self.memread(instr.operands[0]) != 0 {
                            self.instruction_pointer = self.memread(instr.operands[1]);
                        }
                    }
                    JumpIfFalse => {
                        if self.memread(instr.operands[0]) == 0 {
                            self.instruction_pointer = self.memread(instr.operands[1]);
                        }
                    }
                    LessThan => {
                        if self.memread(instr.operands[0]) < self.memread(instr.operands[1]) {
                            self.memwrite(instr.operands[2], 1);
                        } else {
                            self.memwrite(instr.operands[2], 0);
                        }
                    }
                    Equals => {
                        if self.memread(instr.operands[0]) == self.memread(instr.operands[1]) {
                            self.memwrite(instr.operands[2], 1);
                        } else {
                            self.memwrite(instr.operands[2], 0);
                        }
                    }
                    AdjustBase => {
                        self.relative_base += self.memread(instr.operands[0]);
                    }
                    _ => unreachable!(),
                };
            }

            state
        }
    }
}
