use std::env;
use std::io;
use std::io::Read;
use std::fs::File;

const MEMORY_SIZE: usize = 32768;

fn increment_with_overflow(value: &mut u8) {
    if *value == u8::max_value() {
        *value = 0;
    } else {
        *value += 1;
    }
}

fn decrement_with_overflow(value: &mut u8) {
    if *value == 0 {
        *value = u8::max_value();
    } else {
        *value -= 1;
    }
}

fn read_code_from_file(filename: &str) -> String {
    let mut file = File::open(filename)
        .expect("Could not open file");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Could not read file");

    content
}

enum Instruction {
    Increment,
    Decrement,
    MoveRight,
    MoveLeft,
    Putchar,
    Getchar,
    LoopStart,
    LoopEnd
}

impl Instruction {
    fn parse(character: char) -> Option<Instruction> {
        match character {
            '+' => Some(Instruction::Increment),
            '-' => Some(Instruction::Decrement),
            '>' => Some(Instruction::MoveRight),
            '<' => Some(Instruction::MoveLeft),
            '.' => Some(Instruction::Putchar),
            ',' => Some(Instruction::Getchar),
            '[' => Some(Instruction::LoopStart),
            ']' => Some(Instruction::LoopEnd),
            _ => None
        }
    }
}

struct Interpreter {
    memory: [u8; MEMORY_SIZE],
    pointer: usize,
    stack: Vec<usize>
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            memory: [0; MEMORY_SIZE],
            pointer: 0,
            stack: vec![]
        }
    }

    fn parse(code: &str) -> Vec<Instruction> {
        let mut instructions = vec![];

        for character in code.chars() {
            if let Some(instruction) = Instruction::parse(character) {
                instructions.push(instruction);
            }
        }

        instructions
    }

    fn execute(&mut self, instructions: &Vec<Instruction>) {
        let mut index = 0;

        while index < instructions.len() {
            match instructions[index] {
                Instruction::Increment => increment_with_overflow(&mut self.memory[self.pointer]),
                Instruction::Decrement => decrement_with_overflow(&mut self.memory[self.pointer]),
                Instruction::MoveLeft => self.pointer -= 1,
                Instruction::MoveRight => self.pointer += 1,
                Instruction::Putchar => print!("{}", self.memory[self.pointer] as char),
                Instruction::Getchar => self.memory[self.pointer] = io::stdin().bytes().next().unwrap().unwrap(), // NIY
                Instruction::LoopStart => {
                    if self.memory[self.pointer] != 0 {
                        self.stack.push(index);
                    } else {
                        let mut loops_counter = 0;
                        index += 1;

                        loop {
                            match instructions[index] {
                                Instruction::LoopStart => {
                                    loops_counter += 1;
                                },
                                Instruction::LoopEnd => {
                                    if loops_counter == 0 {
                                        break;
                                    }
                                    
                                    loops_counter -= 1;
                                },
                                _ => ()
                            }
                            
                            index += 1;
                        }
                    }
                },
                Instruction::LoopEnd => {
                    index = match self.stack.pop() {
                        Some(idx) => idx - 1, // Index will be incremented
                        None => panic!("Invalid code!")
                    }
                }
            }

            index += 1;
        }
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Incorrect usage!");
        println!("Use: {} file.bf", args[0]);
    } else {
        let code = read_code_from_file(&args[1]);

        let mut interpreter = Interpreter::new();
        interpreter.execute(&Interpreter::parse(&code));
    }
}
