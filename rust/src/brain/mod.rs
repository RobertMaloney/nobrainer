use std::io;
use std::fs::File;
use std::io::prelude::*;

mod program;
use brain::program::Program;

pub struct Brain {
  memory: [u8; 30_000],
  pointer: usize,
  program_counter: usize
}

impl Brain {
  fn new() -> Brain {
    Brain {
      memory: [0 as u8; 30_000],
      pointer: 0 as usize,
      program_counter: 0 as usize
    }
  }

  fn execute(&mut self, instruction: &char, program: &Program) {
    // print!("{} ", *instruction);
    match *instruction {
      '>' => self.pointer += 1,
      '<' => self.pointer -= 1,
      '+' => self.memory[self.pointer] += 1,
      '-' => self.memory[self.pointer] -= 1,
      '.' => {
        print!("{}", self.memory[self.pointer] as char);
        io::stdout().flush().unwrap();
      },
      ',' => {
        let mut buf = [0 as u8; 1];
        let count = io::stdin().read(&mut buf).unwrap();
        if count > 0 {
          self.memory[self.pointer] = buf[0] as u8;
        }
      },
      '[' => {
        if self.memory[self.pointer] == 0 {
          program.end_loop(&mut self.program_counter);
        }
      },
      ']' => {
        if self.memory[self.pointer] != 0 {
          program.start_loop(&mut self.program_counter);
        }
      },
      _ => println!("The instruction '{}' is not supported.", *instruction),
    }
  }

  pub fn interpret(program: &String) {
    let prog = Program::from_string(program);
    // prog.print();
    let mut brain = Brain::new();
    loop {
      match prog.grab_instruction(&brain.program_counter) {
        Some(i) => {
          brain.execute(&i, &prog);
          brain.program_counter += 1;
        },
        None => break
      }
    }
  }

  pub fn generate(text: &String) {
    println!("Generating BrainFuck for the text: {}" , text);
    let mut f = File::create("bf.txt").unwrap();
    let mut v: Vec<u8> = Vec::new();
    let mut size = 0;
    for c in text.chars() {
      let c = c as u8;
      for _ in 0..c {
        v.push('+' as u8);
      }
      v.push('>' as u8);
      v.push('\n' as u8);
      size += 1;
    }
    for _ in 0..size {
      v.push('<' as u8);
    }
    f.write_all(&v).unwrap();
  }
}
