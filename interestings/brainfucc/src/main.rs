
mod bf_engine;

use bf_engine::BrainfuckEngine;

fn main() {
  
  // get source
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
      println!("You must specify a source file.");
      return;
  }
  let source_path = args.get(1).unwrap();
  let source = match std::fs::read_to_string(source_path) {
      Ok(x) => x,
      _ => {
          println!("Error reading specified file.");
          return;
      }
  };
  
  let eng = BrainfuckEngine::new();
  eng.execute(&source);
}