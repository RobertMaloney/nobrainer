pub struct Program {
  instructions: Vec<char>
}

impl Program {
  pub fn from_string(p: &String) -> Program {
    let v = vec!['>', '<', '+', '-', '.', ',', '[', ']'];
    let v: Vec<char> = p.chars().filter(|c| v.iter().any(|x| *x == *c)).collect();

    Program {
      instructions: v
    }
  }

  pub fn grab_instruction(&self, pc: &usize) -> Option<char> {
    return if *pc < self.instructions.len() {
      Some(self.instructions[*pc])
    } else {
      None
    }
  }

  pub fn start_loop(&self, pc: &mut usize) {
    let mut counter = 1;
    *pc -= 1;
    while counter > 0  {
      match self.instructions[*pc] {
        ']' => counter += 1,
        '[' => counter -= 1,
        _ => ()
      }
      *pc -= 1;
    }
    *pc += 1;
  }

  pub fn end_loop(&self, pc: &mut usize) {
    let mut counter = 1;
    *pc += 1;
    while counter > 0 {
      match self.instructions[*pc] {
        ']' => counter -= 1,
        '[' => counter += 1,
        _ => ()
      }
      *pc += 1;
    }
    *pc -= 1;
  }

  #[allow(dead_code)]
  pub fn print(&self) {
    for i in self.instructions.iter() {
      print!("{}", i);
    }
  }
}
