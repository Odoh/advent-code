use itertools::Itertools;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::io;

const INSTRUCTION_POINTER_START: Address = 0;

type Address = usize;

pub struct IntcodeComputer {
    memory: Vec<i32>,
    instruction_pointer: Address,
    input: (Sender<i32>, Receiver<i32>),
    output: (Sender<i32>, Receiver<i32>),
}

impl IntcodeComputer {
    pub fn new(memory: Vec<i32>) -> Self {
        IntcodeComputer {
            memory,
            instruction_pointer: INSTRUCTION_POINTER_START,
            input: mpsc::channel(),
            output: mpsc::channel(),
        }
    }

    pub fn from(s: &str) -> Self {
        let memory = s
            .split(",")
            .map(|s| s.parse::<i32>().expect(s))
            .collect::<Vec<i32>>();
        IntcodeComputer::new(memory)
    }

    pub fn input(&self, input: i32) {
        let (tx, _) = &self.input;
        tx.send(input).expect("Input value was sent")
    }

    pub fn output(&self) -> i32 {
        let (_, rx) = &self.output;
        rx.recv().expect("Output value recieved")
    }

    pub fn run(&mut self) {
        loop {
            let instruction = Instruction::from(self.memory[self.instruction_pointer]);
            if instruction.op_code == OpCode::Halt {
                return
            }

            self.execute(&instruction);
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        let ip = self.instruction_pointer;
//        println!("IP {} Int {:?} = {:?}", ip, &self.memory[ip..ip + instruction.op_code.len()], instruction);

        match instruction.op_code {
            OpCode::Addition => {
                let (p1, p2, p3) = (1..=3).map(|i| self.memory[ip + i]).tuples().next().unwrap();
                let val1 = self.parameter_value(instruction.parameter_modes[0], p1);
                let val2 = self.parameter_value(instruction.parameter_modes[1], p2);
                let write_addr = p3 as usize;

                self.memory[write_addr] = val1 + val2;
                self.instruction_pointer += instruction.op_code.len();
            },
            OpCode::Multiplication => {
                let (p1, p2, p3) = (1..=3).map(|i| self.memory[ip + i]).tuples().next().unwrap();
                let val1 = self.parameter_value(instruction.parameter_modes[0], p1);
                let val2 = self.parameter_value(instruction.parameter_modes[1], p2);
                let write_addr = p3 as usize;

                self.memory[write_addr] = val1 * val2;
                self.instruction_pointer += instruction.op_code.len();
            },
            OpCode::Input => {
                let p1 = self.memory[ip + 1];
                let write_addr = p1 as usize;

                let (_, rx) = &self.input;
                let val = rx.recv().expect("Output should be received");

                self.memory[write_addr] = val;
                self.instruction_pointer += instruction.op_code.len();
            },
            OpCode::Output => {
                let p1 = self.memory[ip + 1];
                let val = self.parameter_value(instruction.parameter_modes[0], p1);

                let (tx, _) = &self.output;
                tx.send(val).expect("Output should be sent");

                self.instruction_pointer += instruction.op_code.len();
            },
            OpCode::JumpIfTrue => {
                let (p1, p2) = (1..=2).map(|i| self.memory[ip + i]).tuples().next().unwrap();
                let val1 = self.parameter_value(instruction.parameter_modes[0], p1);
                let val2 = self.parameter_value(instruction.parameter_modes[1], p2);

                self.instruction_pointer = if val1 != 0 {
                    val2 as usize
                } else {
                    self.instruction_pointer + instruction.op_code.len()
                };
            },
            OpCode::JumpIfFalse => {
                let (p1, p2) = (1..=2).map(|i| self.memory[ip + i]).tuples().next().unwrap();
                let val1 = self.parameter_value(instruction.parameter_modes[0], p1);
                let val2 = self.parameter_value(instruction.parameter_modes[1], p2);

                self.instruction_pointer = if val1 == 0 {
                    val2 as usize
                } else {
                    self.instruction_pointer + instruction.op_code.len()
                };
            },
            OpCode::LessThan => {
                let (p1, p2, p3) = (1..=3).map(|i| self.memory[ip + i]).tuples().next().unwrap();
                let val1 = self.parameter_value(instruction.parameter_modes[0], p1);
                let val2 = self.parameter_value(instruction.parameter_modes[1], p2);
                let write_addr = p3 as usize;


                self.memory[write_addr] = if val1 < val2 { 1 } else { 0 };
                self.instruction_pointer += instruction.op_code.len();
            },
            OpCode::Equals => {
                let (p1, p2, p3) = (1..=3).map(|i| self.memory[ip + i]).tuples().next().unwrap();
                let val1 = self.parameter_value(instruction.parameter_modes[0], p1);
                let val2 = self.parameter_value(instruction.parameter_modes[1], p2);
                let write_addr = p3 as usize;


                self.memory[write_addr] = if val1 == val2 { 1 } else { 0 };
                self.instruction_pointer += instruction.op_code.len();
            },
            OpCode::Halt => (),
        }
    }

    fn parameter_value(&self, parameter_mode: ParameterMode, parameter: i32) -> i32 {
        match parameter_mode {
            ParameterMode::Position => self.memory[parameter as usize],
            ParameterMode::Immediate => parameter,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    op_code: OpCode,
    parameter_modes: Vec<ParameterMode>,
}

impl Instruction {
    pub fn new(op_code: OpCode, parameter_modes: Vec<ParameterMode>) -> Self {
        Instruction {
            op_code,
            parameter_modes,
        }
    }

    pub fn from(int_instruction: i32) -> Self {
        // Instruction Format: ABCDE 1002
        //     DE - two-digit opcode,      02 == opcode 2
        //     C - mode of 1st parameter,  0 == position mode
        //     B - mode of 2nd parameter,  1 == immediate mode
        //     A - mode of 3rd parameter,  0 == position mode,
        let op_code = OpCode::from(int_instruction % 100);
        let param_1_mode = ParameterMode::from((int_instruction / 100) % 10);
        let param_2_mode = ParameterMode::from((int_instruction / 1000) % 10);
        let param_3_mode = ParameterMode::from(int_instruction / 10000);
        let param_modes = vec![param_1_mode, param_2_mode, param_3_mode];

        // "Parameters that an instruction writes to will never be in immediate mode."
        assert!(if op_code == OpCode::Addition { param_3_mode == ParameterMode::Position } else { true });
        assert!(if op_code == OpCode::Multiplication { param_3_mode == ParameterMode::Position } else { true });
        assert!(if op_code == OpCode::Input { param_1_mode == ParameterMode::Position } else { true });
        assert!(if op_code == OpCode::LessThan { param_3_mode == ParameterMode::Position } else { true });
        assert!(if op_code == OpCode::Equals { param_3_mode == ParameterMode::Position } else { true });

        Instruction::new(op_code, param_modes[..(op_code.len() - 1)].to_vec())
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    pub fn from(code: i32) -> Self {
        match code {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Unhandled ParameterMode {}", code),
        }
    }
}


#[derive(PartialEq, Copy, Clone, Debug)]
enum OpCode {
    Addition,
    Multiplication,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

impl OpCode {
    pub fn from(code: i32) -> Self {
        match code {
            1 => OpCode::Addition,
            2 => OpCode::Multiplication,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            99 => OpCode::Halt,
            _ => panic!("Unhandled OpCode {}", code),
        }
    }

    pub fn len(self) -> usize {
        match self {
            OpCode::Addition => 4,
            OpCode::Multiplication => 4,
            OpCode::Input => 2,
            OpCode::Output => 2,
            OpCode::JumpIfTrue => 3,
            OpCode::JumpIfFalse => 3,
            OpCode::LessThan => 4,
            OpCode::Equals => 4,
            OpCode::Halt => 1,
        }
    }
}
