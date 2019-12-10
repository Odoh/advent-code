use itertools::Itertools;
use tokio;
use std::cell::RefCell;
use advent::InputSnake;
use advent::cpu::IntcodeComputer;

const INITIAL_INPUT_SIGNAL: i32 = 0;

pub struct Amplifier {
    // Amplifiers are CPUs which take in two inputs:
    // 1. a phase
    // 2. an input signal
    // Then an output signal is returned
    cpu: Option<IntcodeComputer>,
    phase: i32,
}

impl Amplifier {
    pub fn from(memory: &str, phase: i32) -> Self {
        let mut cpu = IntcodeComputer::from(memory);
        cpu.send_input(phase);
        Amplifier {
            cpu: Some(cpu),
            phase,
        }
    }

    pub fn phase(&self) -> i32 {
        self.phase
    }

    /// Run an amplifier using the given input_signal and returning the final output signal
    pub fn run_with_signal(mut self, input_signal: i32) -> i32 {
        self.send_signal(input_signal);
        self = futures::executor::block_on(self.run());
        self.output_signal()
    }


    /// Synchronously send a signal to the Amplifier
    pub fn send_signal(&mut self, input_signal: i32) {
        self.cpu.as_mut().unwrap().send_input(input_signal);
    }

    /// Synchronously get the output signal of the Amplifier
    pub fn output_signal(&mut self) -> i32 {
        self.cpu.as_mut().unwrap().recv_output()
    }

    /// HACK - synchronously get an unprocessed input signal
    pub fn input_signal(&mut self) -> i32 {
        self.cpu.as_mut().unwrap().recv_input()
    }

    /// Connect the output signal of this amplifier to the input signal of `other_amplifier`.
    pub fn connect_to(&mut self, other_amplifier: &Amplifier) {
        self.cpu.as_mut().unwrap().pipe_to(&other_amplifier.cpu.as_ref().unwrap())
    }

    /// Asynchronously run the amplifier
    pub async fn run(mut self) -> Self {
        let mut cpu = self.cpu.take().unwrap();
        cpu = cpu.run().await;

        self.cpu = Some(cpu);
        self
    }
}

/// HACK - to connect two amplifiers from the same Vector
fn connect(src_amp: &RefCell<Amplifier>, dst_amp: &RefCell<Amplifier>) {
    src_amp.borrow_mut().connect_to(&dst_amp.borrow())
}

fn run_amplifiers(amplifiers: Vec<Amplifier>, initial_input_signal: i32) -> i32 {
    let mut signal = initial_input_signal;
    for amplifier in amplifiers.into_iter() {
        signal = amplifier.run_with_signal(signal);
    }
    signal
}

fn run_amplifiers_with_feedback(mut amplifiers: Vec<Amplifier>, initial_input_signal: i32) -> i32 {
    let mut runtime = tokio::runtime::Runtime::new().expect("runtime to initialize");

    // create connections between adjacent amplifiers + the last to first amplifier
    let amplifier_rcs: Vec<RefCell<Amplifier>> = amplifiers.into_iter().map(|amp| RefCell::new(amp)).collect();
    (0..(amplifier_rcs.len() - 1)).for_each(|i| {
        connect(amplifier_rcs.get(i).unwrap(), amplifier_rcs.get(i + 1).unwrap());
    });
    connect(amplifier_rcs.last().unwrap(), amplifier_rcs.first().unwrap());
    amplifiers = amplifier_rcs.into_iter().map(|amp_rc| amp_rc.into_inner()).collect();

    // sent the initial input signal then run each amplifier
    amplifiers.first_mut().expect("Expect at least one amplifier").send_signal(initial_input_signal);
    amplifiers = runtime.block_on(async {
        futures::future::join_all(amplifiers.into_iter().map(|amplifier| amplifier.run())).await
    });

    // due to the last amplifier being connected to the first amplifier, its final output_signal
    // will have been sent to the input of the first amplifier
    amplifiers.first_mut().expect("Expect at least one amplifier").input_signal()
}

fn part_one() {
    let input = InputSnake::new("input");
    let memory = input.no_snake();

    let signal = (0..=4).permutations(5)
        .map(|phases| phases.iter()
            .map(|phase| Amplifier::from(&memory, *phase))
            .collect::<Vec<Amplifier>>())
        .map(|amplifiers| run_amplifiers(amplifiers, INITIAL_INPUT_SIGNAL))
        .max()
        .expect("Expect at least one signal");

    println!("Part One: {:?}", signal);
}

fn part_two() {
    let input = InputSnake::new("input");
    let memory = input.no_snake();

    let signal = (5..=9).permutations(5)
        .map(|phases| phases.iter()
            .map(|phase| Amplifier::from(&memory, *phase))
            .collect::<Vec<Amplifier>>())
        .map(|amplifiers| run_amplifiers_with_feedback(amplifiers, INITIAL_INPUT_SIGNAL))
        .max()
        .expect("Expect at least one signal");

    println!("Part Two: {:?}", signal);
}

fn main() {
    env_logger::init();

    part_one();
    part_two();
}
