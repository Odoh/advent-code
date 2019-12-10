use itertools::Itertools;
use tokio::sync::mpsc::{Sender, Receiver};
use tokio::sync::mpsc;
use tokio;
use futures::join;
use log::debug;

const INSTRUCTION_POINTER_START: Address = 0;
const CHANNEL_BUFFER_SIZE: usize = 100;
const SHUTDOWN_SIGNAL: i32 = -123456789;

type Address = usize;

pub struct IntcodeComputer {
    memory: Vec<i32>,
    instruction_pointer: Address,

    // Async channels for sending and receiving Input and Output.
    // The Output of this computer may be piped to the Input of other computers.
    input: (Sender<i32>, Receiver<i32>),
    output: (Sender<i32>, Option<Receiver<i32>>),
    pipe: Option<Pipe>,
}

#[derive(Debug)]
struct Instruction {
    op_code: OpCode,
    parameter_modes: Vec<ParameterMode>,
}


#[derive(PartialEq, Copy, Clone, Debug)]
enum ParameterMode {
    Position,
    Immediate,
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

struct Pipe {
    src_output: Receiver<i32>,
    dst_inputs: Vec<Sender<i32>>,
}

impl IntcodeComputer {
    /// Construct a new computer with the given memory.
    pub fn new(memory: Vec<i32>) -> Self {
        let (input_tx, input_rx) = mpsc::channel(CHANNEL_BUFFER_SIZE);
        let (output_tx, output_rx) = mpsc::channel(CHANNEL_BUFFER_SIZE);
        IntcodeComputer {
            memory,
            instruction_pointer: INSTRUCTION_POINTER_START,
            input: (input_tx, input_rx),
            output: (output_tx, Some(output_rx)),
            pipe: None,
        }
    }

    /// Construct a new computer with the given serialized memory.
    pub fn from(s: &str) -> Self {
        let memory = s
            .split(",")
            .map(|s| s.parse::<i32>().expect(s))
            .collect::<Vec<i32>>();
        IntcodeComputer::new(memory)
    }

    /// Pipe the Output of this computer to the Input of `other_cpu`.
    pub fn pipe_to(&mut self, other_cpu: &IntcodeComputer) {
        if self.pipe.is_none() {
            // the pipe now owns the Output Reciever
            self.pipe = Some(Pipe::new(self.output.1.take().expect("Expect output to exist")))
        }
        self.pipe.as_mut().unwrap().add_pipe(other_cpu.input.0.clone())
    }

    /// Synchronously send `input` to this computer.
    pub fn send_input(&mut self, input: i32) {
        futures::executor::block_on(async {
            let (tx, _) = &mut self.input;
            tx.send(input).await.expect("Input value was sent")
        })
    }

    /// Synchronously receive output from this computer.
    pub fn recv_output(&mut self) -> i32 {
        futures::executor::block_on(async {
            let (_, rx) = &mut self.output;
            rx.as_mut().expect("Output has already been taken").recv().await.expect("Output value recieved")
        })
    }

    /// Synchronously receive any outstanding input to this computer.
    pub fn recv_input(&mut self) -> i32 {
        futures::executor::block_on(async {
            let (_, rx) = &mut self.input;
            rx.recv().await.expect("Output should be received")
        })
    }

    /// Asynchonously run this computer and, if it exists, its pipe.
    pub async fn run(self) -> Self {
        let mut cpu = self;
        if cpu.pipe.is_some() {
            // a pipe exists, give the pipe runtask ownership
            let mut pipe = cpu.pipe.take().unwrap();
            cpu = (async { join!(pipe.run(), cpu.run_cpu()) }.await).1;
            cpu.pipe = Some(pipe);
        } else {
            // no pipe exists exists
            cpu = cpu.run_cpu().await;
        }

        return cpu
    }

    async fn run_cpu(mut self) -> Self {
        loop {
            let instruction = Instruction::from(self.memory[self.instruction_pointer]);
            if instruction.op_code == OpCode::Halt {
                // if a pipe exists, send a shutdown signal to stop the pipe
                if self.output.1.is_none() {
                    futures::executor::block_on(async {
                        let (tx, _) = &mut self.output;
                        tx.send(SHUTDOWN_SIGNAL).await.expect("Output should be sent")
                    });
                }
                return self;
            }

            self.execute(&instruction).await;
        }
    }

    async fn execute(&mut self, instruction: &Instruction) {
        let ip = self.instruction_pointer;
        debug!("IP {} Int {:?} = {:?}", ip, &self.memory[ip..ip + instruction.op_code.len()], instruction);

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

                let (_, rx) = &mut self.input;
                let val = rx.recv().await.expect("Output should be received");

                self.memory[write_addr] = val;
                self.instruction_pointer += instruction.op_code.len();
            },
            OpCode::Output => {
                let p1 = self.memory[ip + 1];
                let val = self.parameter_value(instruction.parameter_modes[0], p1);

                let (tx, _) = &mut self.output;
                tx.send(val).await.expect("Output should be sent");

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

impl ParameterMode {
    pub fn from(code: i32) -> Self {
        match code {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Unhandled ParameterMode {}", code),
        }
    }
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

impl Pipe {
    pub fn new(src_output: Receiver<i32>) -> Self {
        Pipe {
            src_output,
            dst_inputs: Vec::new(),
        }
    }

    pub fn add_pipe(&mut self, dst_input: Sender<i32>) {
        self.dst_inputs.push(dst_input.clone())
    }

    pub async fn run(&mut self) {
        loop {
            if let Some(value) = self.src_output.recv().await {
                if value == SHUTDOWN_SIGNAL {
                    debug!("Pipe shutdown");
                    return
                }

                debug!("Pipe sending: src -> {} -> dst", value);
                for dst_input in self.dst_inputs.iter_mut() {
                    dst_input.send(value).await.expect("Expect dst send to succeed");
                }
            } else {
                debug!("Src program closed its output channel: stopping the pipe");
                return
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use tokio;
    use futures::join;

    #[test]
    fn test_pipe() {
        let mut runtime = tokio::runtime::Runtime::new().expect("runtime to initialize");
        runtime.block_on(async {
            let mut src_cpu = IntcodeComputer::from("4,0,4,0,99");
            let mut dst_cpu = IntcodeComputer::from("3,1,3,2,99");

            src_cpu.pipe_to(&mut dst_cpu);
            tokio::spawn(async { join!(src_cpu.run(), dst_cpu.run()) });
        });
    }
}
