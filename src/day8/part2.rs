use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::hash_map::Values;

/// A CPU which consists of a set of registers.
struct Cpu {
    /// Registers stored by name.
    registers: HashMap<String, Register>,

    /// The largest register value seen.
    largest_value: i32,
}

/// A Register in the CPU.
struct Register {
    value: i32,
}

/// Modification instructions.
enum Modification {
    Inc(i32),
    Dec(i32),
}

/// A condition for executing an instruction.
enum Condition {
    LessThan(i32),
    GreaterThan(i32),
    LessThanOrEqual(i32),
    GreaterThanOrEqual(i32),
    Equal(i32),
    NotEqual(i32),
}

/// A CPU instruction.
struct Instruction {
    check_register: String,
    condition: Condition,
    modify_register: String,
    modification: Modification,
}

impl Register {

    /// Whether the specified condition is true or false on a given register.
    pub fn is_condition(&self, condition: &Condition) -> bool {
        match condition {
            &Condition::LessThan(i) => self.value < i,
            &Condition::GreaterThan(i) => self.value > i,
            &Condition::LessThanOrEqual(i) => self.value <= i,
            &Condition::GreaterThanOrEqual(i) => self.value >= i,
            &Condition::Equal(i) => self.value == i,
            &Condition::NotEqual(i) => self.value != i,
        }
    }

    /// Modify the register with the specified modification.
    pub fn modify(&mut self, modification: &Modification) {
        match modification {
            &Modification::Inc(i) => self.value += i,
            &Modification::Dec(i) => self.value -= i,
        };
    }
}

type RegisterIter<'a> = Values<'a, String, Register>;

impl Cpu {

    /// Create a CPU with the specified registers.
    pub fn new(registers: HashMap<String, Register>) -> Self {
        Cpu { registers,
              largest_value: 0, }
    }

    /// Execution a sequence of instructions.
    pub fn execute(&mut self, instructions: &[Instruction]) {
        for i in instructions {
            let condition = {
                // return immutable borrow before mutable borrow
                let check = self.registers.get(&i.check_register).unwrap();
                check.is_condition(&i.condition)   
            };
            if condition {
                let modify = self.registers.get_mut(&i.modify_register).unwrap();
                modify.modify(&i.modification); 

                if modify.value > self.largest_value {
                    self.largest_value = modify.value;
                }
            }
        }
    }

    /// An iterator over the registers.
    pub fn registers(&self) -> RegisterIter {
        self.registers.values()
    }

    /// The largest register value seen during execution.
    pub fn largest_seen_register_value(&self) -> i32 {
        self.largest_value
    }
}

fn parse_file(filename: &str,
              registers: &mut HashMap<String, Register>,
              instructions: &mut Vec<Instruction>) {
    let file = File::open(filename).expect("file not found");
    for line in BufReader::new(&file).lines()
                                     .filter_map(Result::ok) {
        parse_line(&line, registers, instructions);
    }
}

fn parse_line(line: &str, 
              registers: &mut HashMap<String, Register>,
              instructions: &mut Vec<Instruction>) {
    // all tokens are whitespace delimited:
    let tokens = line.split_whitespace().collect::<Vec<&str>>();
    // "b inc 5 if a > 1"
    // "0 1   2 3  4 5 6"
    let modify_register = tokens[0];
    let modification = match (tokens[1], tokens[2].parse::<i32>()) {
        ("inc", Ok(i)) => Modification::Inc(i),
        ("dec", Ok(i)) => Modification::Dec(i),
        (_, Err(_)) => panic!("Unable to parse into i32: {}", tokens[2]),
        (m, _) => panic!("Unknown modification token: {}", m),
    };
    let check_register = tokens[4];
    let condition = match (tokens[5], tokens[6].parse::<i32>()) {
        ("<", Ok(i)) => Condition::LessThan(i),
        (">", Ok(i)) => Condition::GreaterThan(i),
        ("<=", Ok(i)) => Condition::LessThanOrEqual(i),
        (">=", Ok(i)) => Condition::GreaterThanOrEqual(i),
        ("==", Ok(i)) => Condition::Equal(i),
        ("!=", Ok(i)) => Condition::NotEqual(i),
        (_, Err(_)) => panic!("Unable to parse into i32: {}", tokens[6]),
        (c, _) => panic!("Unknown condition token: {}", c),
    };

    // store found registers
    registers.insert(modify_register.to_string(),
                     Register { value: 0 });
    registers.insert(check_register.to_string(),
                     Register { value: 0 });

    // add instruction
    instructions.push(Instruction {
        modify_register: modify_register.to_string(),
        modification: modification,
        check_register: check_register.to_string(),
        condition: condition,
    });
}

fn main() {
    // read input from file
    let mut registers: HashMap<String, Register> = HashMap::new();
    let mut instructions: Vec<Instruction> = Vec::new();
    parse_file("question", &mut registers, &mut instructions);

    let mut cpu = Cpu::new(registers);
    cpu.execute(&instructions);

    let largest_value = cpu.registers()
                           .map(|r| r.value)
                           .max()
                           .unwrap();
    let largest_seen_value = cpu.largest_seen_register_value();
    println!("largest_value [{}] largest_seen_value [{}]", largest_value, largest_seen_value);
}
