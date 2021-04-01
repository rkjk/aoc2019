pub struct Instr {
    pub instr: Vec<i64>,
    pub input: Vec<i64>,
    pub output: Vec<i64>,
    pub relative_base: i64,
}

#[derive(Debug)]
pub enum Mode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug)]
pub enum Opcode {
    Add,
    Mul,
    Inp,
    Outp,
    Halt,
    JmpIfTrue,
    JmpIfFalse,
    LessThan,
    Equals,
    ChBase,
}

pub type ModeSet = (Mode, Mode, Mode);

impl Instr {
    pub fn iterate(&mut self) {
        let mut i: usize = 0;
        while i < self.instr.len() {
            let opcode = match self.instr[i] % 100 {
                1 => Opcode::Add,
                2 => Opcode::Mul,
                3 => Opcode::Inp,
                4 => Opcode::Outp,
                5 => Opcode::JmpIfTrue,
                6 => Opcode::JmpIfFalse,
                7 => Opcode::LessThan,
                8 => Opcode::Equals,
                9 => Opcode::ChBase,
                99 => Opcode::Halt,
                _ => panic!("opcode out of range"),
            };
            let param_mode: ModeSet = match self.instr[i] / 100 {
                0 => (Mode::Position, Mode::Position, Mode::Position),
                1 => (Mode::Immediate, Mode::Position, Mode::Position),
                10 => (Mode::Position, Mode::Immediate, Mode::Position),
                11 => (Mode::Immediate, Mode::Immediate, Mode::Position),
                2 => (Mode::Relative, Mode::Position, Mode::Position),
                12 => (Mode::Relative, Mode::Immediate, Mode::Position),
                20 => (Mode::Position, Mode::Relative, Mode::Position),
                21 => (Mode::Immediate, Mode::Relative, Mode::Position),
                22 => (Mode::Relative, Mode::Relative, Mode::Position),
                200 => (Mode::Position, Mode::Position, Mode::Relative),
                201 => (Mode::Immediate, Mode::Position, Mode::Relative),
                210 => (Mode::Position, Mode::Immediate, Mode::Relative),
                211 => (Mode::Immediate, Mode::Immediate, Mode::Relative),
                202 => (Mode::Relative, Mode::Position, Mode::Relative),
                220 => (Mode::Position, Mode::Relative, Mode::Relative),
                212 => (Mode::Relative, Mode::Immediate, Mode::Relative),
                221 => (Mode::Immediate, Mode::Relative, Mode::Relative),
                222 => (Mode::Relative, Mode::Relative, Mode::Relative),
                _ => panic!("parameter mode out of range"),
            };

            //println!("{} {} {:?} {:?}", i, self.instr[i], opcode, param_mode);

            match opcode {
                Opcode::Add => {
                    self.add(
                        self.instr[i + 1],
                        self.instr[i + 2],
                        self.instr[i + 3],
                        &param_mode,
                    );
                    i += 4
                }
                Opcode::Mul => {
                    self.multiply(
                        self.instr[i + 1],
                        self.instr[i + 2],
                        self.instr[i + 3],
                        &param_mode,
                    );
                    i += 4;
                }
                Opcode::Inp => {
                    self.get_input(self.instr[(i + 1) as usize] as usize, &param_mode);
                    i += 2;
                }
                Opcode::Outp => {
                    let out = self.store_output(self.instr[i + 1] as usize, &param_mode);
                    self.output.push(out);
                    i += 2;
                }
                Opcode::JmpIfTrue => {
                    let (op1, op2) =
                        self.get_args(self.instr[i + 1], self.instr[i + 2], &param_mode);
                    match op1 == 0 {
                        false => i = op2 as usize,
                        true => i += 3,
                    };
                }
                Opcode::JmpIfFalse => {
                    let (op1, op2) =
                        self.get_args(self.instr[i + 1], self.instr[i + 2], &param_mode);
                    match op1 != 0 {
                        false => i = op2 as usize,
                        true => i += 3,
                    };
                }
                Opcode::LessThan => {
                    self.less_than(
                        self.instr[i + 1],
                        self.instr[i + 2],
                        self.instr[i + 3],
                        &param_mode,
                    );
                    i += 4;
                }
                Opcode::Equals => {
                    self.equals(
                        self.instr[i + 1],
                        self.instr[i + 2],
                        self.instr[i + 3],
                        &param_mode,
                    );
                    i += 4;
                }
                Opcode::ChBase => {
                    match param_mode.0 {
                        Mode::Position => {
                            self.relative_base += self.instr[self.instr[i + 1] as usize]
                        }
                        Mode::Immediate => self.relative_base += self.instr[i + 1],
                        Mode::Relative => {
                            self.relative_base +=
                                self.instr[(self.relative_base + self.instr[i + 1]) as usize]
                        }
                    };
                    //self.relative_base += self.instr[i+1];
                    i += 2;
                }
                Opcode::Halt => break,
            };
        }
    }

    fn get_args(&self, inp1: i64, inp2: i64, modes: &ModeSet) -> (i64, i64) {
        let op1 = match modes.0 {
            Mode::Position => self.instr[inp1 as usize],
            Mode::Immediate => inp1,
            Mode::Relative => self.instr[(inp1 + self.relative_base) as usize],
        };
        let op2 = match modes.1 {
            Mode::Position => self.instr[inp2 as usize],
            Mode::Immediate => inp2,
            Mode::Relative => self.instr[(inp2 + self.relative_base) as usize],
        };
        (op1, op2)
    }

    fn set_output(&mut self, out: i64, val: i64, modes: &ModeSet) {
        // Different from function store_output => this is a helper function that will be used by
        // add, multiply, equals, less_than to set output
        let out_index = match modes.2 {
            Mode::Position => out as usize,
            Mode::Relative => (out + self.relative_base) as usize,
            Mode::Immediate => panic!("Cannot output in immediate mode"),
        };
        if out_index > self.instr.len() {
            self.instr.resize(out_index * 2, 0);
        };
        self.instr[out_index] = val;
    }

    fn less_than(&mut self, inp1: i64, inp2: i64, out: i64, modes: &ModeSet) {
        let (op1, op2) = self.get_args(inp1, inp2, modes);
        match op1 < op2 {
            true => self.set_output(out, 1, modes),
            false => self.set_output(out, 0, modes),
        };
    }

    fn equals(&mut self, inp1: i64, inp2: i64, out: i64, modes: &ModeSet) {
        let (op1, op2) = self.get_args(inp1, inp2, modes);
        match op1 == op2 {
            true => self.set_output(out, 1, modes),
            false => self.set_output(out, 0, modes),
        };
    }

    fn add(&mut self, inp1: i64, inp2: i64, out: i64, modes: &ModeSet) {
        let (op1, op2) = self.get_args(inp1, inp2, modes);
        self.set_output(out, op1 + op2, modes);
    }

    fn multiply(&mut self, inp1: i64, inp2: i64, out: i64, modes: &ModeSet) {
        let (op1, op2) = self.get_args(inp1, inp2, modes);
        self.set_output(out, op1 * op2, modes);
    }

    fn get_input(&mut self, pos: usize, modes: &ModeSet) {
        let inp_int = self.input.pop().expect("Expected input");

        match modes.0 {
            Mode::Position => {
                self.instr[pos] = inp_int;
            }
            Mode::Relative => {
                self.instr[pos + (self.relative_base as usize)] = inp_int;
            }
            _ => panic!("Immediate Mode should not exist for input"),
        };
    }

    fn store_output(&mut self, out: usize, modes: &ModeSet) -> i64 {
        match modes.0 {
            Mode::Position => self.instr[out],
            Mode::Immediate => out as i64,
            Mode::Relative => self.instr[((out as i64) + self.relative_base) as usize],
        }
    }
}
