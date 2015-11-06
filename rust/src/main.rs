use std::env;
use std::fs::File;
use std::io::prelude::*;

mod brain;
use brain::Brain;

fn grab_file_contents(path: &str) -> String {
  let mut f = File::open(path).unwrap();
  let mut s = String::new();
  f.read_to_string(&mut s).unwrap();

  return s;
}

fn main() {
  let args: Vec<_> = env::args().collect();
  if args.len() < 3 {
    println!("Not enough arguments!");
    return;
  }

  match args[1].as_ref() {
    "generate" => {
      Brain::generate(&args[2]);
    },
    "interpret" => {
      let program = grab_file_contents(&args[2]);
      Brain::interpret(&program);
    },
    _ => println!("Operation not supported.")
  }
}
