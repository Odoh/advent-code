use itertools::Itertools;
use tokio::sync::mpsc::{Sender, Receiver};
use tokio::sync::mpsc;
use tokio;
use log::debug;

const INSTRUCTION_POINTER_START: Address = 0;
const RELATIVE_BASE_START: Address = 0;
const CHANNEL_BUFFER_SIZE: usize = 100;
const SHUTDOWN_SIGNAL: i64 = -123456789;

type Address = usize;

pub struct IntcodeComputer {
    memory: Vec<i64>,
    instruction_pointer: Address,
    relative_base: Address,

    // Async channels for sending and receiving Input and Output.
    // The Output of this computer may be piped to the Input of other computers.
    input: (Option<Sender<i64>>, Receiver<i64>),
    output: (Sender<i64>, Option<Receiver<i64>>),
    pipe: Option<Pipe>,
}

pub struct Proxy {
    input_proxy: InputProxy,
    output_proxy: OutputProxy,
}

#[derive(Clone)]
pub struct InputProxy {
    input_tx: Sender<i64>,
}

pub struct OutputProxy {
    output_rx: Receiver<i64>,
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
    Relative,
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
    AdjustRelativeBase,
    Halt,
}

struct Pipe {
    src_output: Receiver<i64>,
    dst_inputs: Vec<Sender<i64>>,
}

impl IntcodeComputer {
    /// Construct a new computer with the given memory.
    pub fn new(memory: Vec<i64>) -> Self {
        let (input_tx, input_rx) = mpsc::channel(CHANNEL_BUFFER_SIZE);
        let (output_tx, output_rx) = mpsc::channel(CHANNEL_BUFFER_SIZE);
        IntcodeComputer {
            memory,
            relative_base: RELATIVE_BASE_START,
            instruction_pointer: INSTRUCTION_POINTER_START,
            input: (Some(input_tx), input_rx),
            output: (output_tx, Some(output_rx)),
            pipe: None,
        }
    }

    /// Construct a new computer with the given serialized memory.
    pub fn from(s: &str) -> Self {
        let memory = s
            .split(",")
            .map(|s| s.parse::<i64>().expect(s))
            .collect::<Vec<i64>>();
        IntcodeComputer::new(memory)
    }

    /// Set the memory at the given address.
    pub fn set_memory(&mut self, address: Address, value: i64) {
        self.ensure_memory_exists(address);
        self.memory[address] = value;
    }

    /// Pipe the Output of this computer to the Input of `other_cpu`.
    pub fn pipe_to(&mut self, other_cpu: &IntcodeComputer) {
        if self.pipe.is_none() {
            // the pipe now owns the Output Reciever
            self.pipe = Some(Pipe::new(self.output.1.take().expect("Expect output to exist")))
        }
        self.pipe.as_mut().unwrap().add_pipe(other_cpu.input.0.as_ref().unwrap().clone())
    }

    /// Return a proxy which forwards to the input and output of this computer.
    pub fn proxy(&mut self) -> Proxy {
        let input_proxy = self.input_proxy();
        let output_proxy = self.output_proxy();
        Proxy::new(input_proxy, output_proxy)
    }

    /// Return a proxy which forwards to the input of this computer.
    pub fn input_proxy(&mut self) -> InputProxy {
        let input_tx = self.input.0.take().expect("Expect input to not be taken");
        InputProxy::new(input_tx)
    }

    /// Return a proxy which forwards to the output of this computer.
    pub fn output_proxy(&mut self) -> OutputProxy {
        let output_rx = self.output.1.take().expect("Expect output to not be taken");
        OutputProxy::new(output_rx)
    }

    /// Synchronously send `input` to this computer.
    pub fn send_input(&mut self, input: i64) {
        futures::executor::block_on(async {
            let (tx, _) = &mut self.input;
            tx.as_mut().expect("Input has already been taken").send(input).await.expect("Input value was sent")
        })
    }

    /// Synchronously receive output from this computer.
    pub fn recv_output(&mut self) -> i64 {
        futures::executor::block_on(async {
            let (_, rx) = &mut self.output;
            rx.as_mut().expect("Output has already been taken").recv().await.expect("Output value recieved")
        })
    }

    /// Synchronously receive any outstanding input to this computer.
    pub fn recv_input(&mut self) -> i64 {
        futures::executor::block_on(async {
            let (_, rx) = &mut self.input;
            rx.recv().await.expect("Output should be received")
        })
    }

    /// Asynchonously run this computer and, if it exists, its pipe.
    pub async fn run(&mut self) {
        if self.pipe.is_some() {
            // a pipe exists, give the pipe runtask ownership
            let mut pipe = self.pipe.take().unwrap();
            futures::future::join(pipe.run(), self.run_cpu()).await;
            self.pipe = Some(pipe);
        } else {
            // no pipe exists exists
            self.run_cpu().await;
        }
    }

    async fn run_cpu(&mut self) {
        loop {
//            debug!("IP {} RB {} Memory {:?}", self.instruction_pointer, self.relative_base, self.memory);
            let instruction = Instruction::from(self.memory[self.instruction_pointer]);
            if instruction.op_code == OpCode::Halt {
                // if a pipe or proxy exists exists, send a shutdown signal to stop them
                if self.output.1.is_none() {
                    futures::executor::block_on(async {
                        let (tx, _) = &mut self.output;
                        tx.send(SHUTDOWN_SIGNAL).await.expect("Output should be sent")
                    });
                }
                return
            }

            self.execute(&instruction).await;
        }
    }

    async fn execute(&mut self, instruction: &Instruction) {
        let ip = self.instruction_pointer;
        debug!("IP {} RB {} Int {:?} = {:?}",
               self.instruction_pointer,
               self.relative_base,
               &self.memory[ip..ip + instruction.op_code.len()],
               instruction);

        // Ensure this operation will have enough memory.
        // Additional checks are performed when the parameter value is read from memory, and when
        // writing to memory
        self.ensure_memory_exists(ip + instruction.op_code.len());

        match instruction.op_code {
            OpCode::Addition => {
                let (p1, p2, p3) = (1..=3).map(|i| self.memory[ip + i]).tuples().next().unwrap();
                let val1 = self.parameter_value(instruction.parameter_modes[0], p1);
                let val2 = self.parameter_value(instruction.parameter_modes[1], p2);
                let write_addr = self.parameter_value_address(instruction.parameter_modes[2], p3);

                self.memory[write_addr] = val1 + val2;
                self.instruction_pointer += instruction.op_code.len();
            },
            OpCode::Multiplication => {
                let (p1, p2, p3) = (1..=3).map(|i| self.memory[ip + i]).tuples().next().unwrap();
                let val1 = self.parameter_value(instruction.parameter_modes[0], p1);
                let val2 = self.parameter_value(instruction.parameter_modes[1], p2);
                let write_addr = self.parameter_value_address(instruction.parameter_modes[2], p3);

                self.memory[write_addr] = val1 * val2;
                self.instruction_pointer += instruction.op_code.len();
            },
            OpCode::Input => {
                let p1 = self.memory[ip + 1];
                let write_addr = self.parameter_value_address(instruction.parameter_modes[0], p1);

                let (_, rx) = &mut self.input;
                let val = rx.recv().await.expect("Input should be received");

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
                let write_addr = self.parameter_value_address(instruction.parameter_modes[2], p3);

                self.memory[write_addr] = if val1 < val2 { 1 } else { 0 };
                self.instruction_pointer += instruction.op_code.len();
            },
            OpCode::Equals => {
                let (p1, p2, p3) = (1..=3).map(|i| self.memory[ip + i]).tuples().next().unwrap();
                let val1 = self.parameter_value(instruction.parameter_modes[0], p1);
                let val2 = self.parameter_value(instruction.parameter_modes[1], p2);
                let write_addr = self.parameter_value_address(instruction.parameter_modes[2], p3);

                self.memory[write_addr] = if val1 == val2 { 1 } else { 0 };
                self.instruction_pointer += instruction.op_code.len();
            },
            OpCode::AdjustRelativeBase => {
                let p1 = self.memory[ip + 1];
                let val = self.parameter_value(instruction.parameter_modes[0], p1);

                if val > 0 {
                    self.relative_base += val as usize;
                } else {
                    self.relative_base -= (val * -1) as usize;
                }
                self.instruction_pointer += instruction.op_code.len();
            }
            OpCode::Halt => (),
        }
    }

    fn parameter_value(&mut self, parameter_mode: ParameterMode, parameter: i64) -> i64 {
        match parameter_mode {
            ParameterMode::Position => {
                let address = self.parameter_value_address(parameter_mode, parameter);
                self.memory[address]
            },
            ParameterMode::Immediate => parameter,
            ParameterMode::Relative => {
                let address = self.parameter_value_address(parameter_mode, parameter);
                self.memory[address]
            }
        }
    }

    fn parameter_value_address(&mut self, parameter_mode: ParameterMode, parameter: i64) -> usize {
        match parameter_mode {
            ParameterMode::Position => {
                let address = parameter as usize;
                self.ensure_memory_exists(address);
                address
            },
            ParameterMode::Relative => {
                if parameter > 0 {
                    let address = self.relative_base + parameter as usize;
                    self.ensure_memory_exists(address);
                    address
                } else {
                    let address = self.relative_base - (parameter * -1) as usize;
                    address
                }
            },
            ParameterMode::Immediate => panic!("Immediate parameter mode cannot be used for addresses")
        }
    }

    /// Allocate additional memory, with 0s, if necessary
    fn ensure_memory_exists(&mut self, address: Address) {
        while address >= self.memory.len() {
            debug!("Increasing memory");
            let mut additional_memory = vec![0; self.memory.len()];
            self.memory.append(&mut additional_memory);
        }
    }
}

impl Proxy {
    fn new(input_proxy: InputProxy, output_proxy: OutputProxy) -> Self {
        Proxy {
            input_proxy,
            output_proxy,
        }
    }

    /// Asynchonrously send `input` to computer behind this proxy.
    pub async fn send(&mut self, input: i64) {
        self.input_proxy.send(input).await;
    }

    /// Asynchronously receive output from this computer behind this proxy.
    /// None returned when the CPU has halted.
    pub async fn recv(&mut self) -> Option<i64> {
        self.output_proxy.recv().await
    }
}

impl InputProxy {
    fn new(input_tx: Sender<i64>) -> Self {
        InputProxy {
            input_tx,
        }
    }

    /// Asynchonrously send `input` to computer behind this proxy.
    pub async fn send(&mut self, input: i64) {
        self.input_tx.send(input).await.expect("Input value was sent");
    }
}

impl OutputProxy {
    fn new(output_rx: Receiver<i64>) -> Self {
        OutputProxy {
            output_rx,
        }
    }

    /// Asynchronously receive output from this computer behind this proxy.
    /// None returned when the CPU has halted.
    pub async fn recv(&mut self) -> Option<i64> {
        let value = self.output_rx.recv().await.expect("Output value recieved");
        if value == SHUTDOWN_SIGNAL {
            debug!("Pipe shutdown");
            return None;
        }
        Some(value)
    }
}

impl Instruction {
    pub fn new(op_code: OpCode, parameter_modes: Vec<ParameterMode>) -> Self {
        Instruction {
            op_code,
            parameter_modes,
        }
    }

    pub fn from(int_instruction: i64) -> Self {
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
        assert!(if op_code == OpCode::Addition { param_3_mode != ParameterMode::Immediate } else { true });
        assert!(if op_code == OpCode::Multiplication { param_3_mode != ParameterMode::Immediate } else { true });
        assert!(if op_code == OpCode::Input { param_1_mode != ParameterMode::Immediate } else { true });
        assert!(if op_code == OpCode::LessThan { param_3_mode != ParameterMode::Immediate } else { true });
        assert!(if op_code == OpCode::Equals { param_3_mode != ParameterMode::Immediate } else { true });

        Instruction::new(op_code, param_modes[..(op_code.len() - 1)].to_vec())
    }
}

impl ParameterMode {
    pub fn from(code: i64) -> Self {
        match code {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("Unhandled ParameterMode {}", code),
        }
    }
}


impl OpCode {
    pub fn from(code: i64) -> Self {
        match code {
            1 => OpCode::Addition,
            2 => OpCode::Multiplication,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            9 => OpCode::AdjustRelativeBase,
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
            OpCode::AdjustRelativeBase => 2,
            OpCode::Halt => 1,
        }
    }
}

impl Pipe {
    pub fn new(src_output: Receiver<i64>) -> Self {
        Pipe {
            src_output,
            dst_inputs: Vec::new(),
        }
    }

    pub fn add_pipe(&mut self, dst_input: Sender<i64>) {
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
            futures::future::join(src_cpu.run(), dst_cpu.run()).await;
        });
    }
}
