use std::io::Read;

pub const BLOCK_SIZE: usize = 30000;

#[derive(PartialEq)]
pub enum EntryType {
  None,
  Number,
  String,
}

pub struct BrainfuckEngine {
  block: [i32; BLOCK_SIZE], // block of i32 (not bytes, i want more power)
  ip: usize,                // instruction pointer in source file
  bp: usize,                // block pointer inside block (current cell index)
  loops: [usize; 255],      // array of nested loop pointers
  lp: usize,                // current loop is lp-1, lp==0 means no current loop
}

impl BrainfuckEngine {
  /// Initialize brainfuck engine.
  pub fn new() -> BrainfuckEngine {
    BrainfuckEngine {
      block: [0; BLOCK_SIZE],
      ip: 0,
      bp: 0,
      loops: [0; 255],
      lp: 0,
    }
  }
  
  /// Execute source string.
  pub fn execute(&mut self, source: &str) {
    let mut line_idx: u64 = 0; // track line index
    let s_len = source.len();
    let chars: Vec<char> = source.chars().collect();
    // TODO: code for parsing args
    // let args_str: &str = &line[2..];
    // let args: Vec<&str> = args_str.split(" ").collect();
    // if (args.len() == 2) {
    //   let from: usize = (args.get(0).unwrap()).parse().unwrap();
    //   let to: usize = (args.get(1).unwrap()).parse().unwrap();
    //   self.display_block(from, to);
    // }
    while self.ip < s_len {
      let instruction = chars.get(self.ip).unwrap();
      // execute instructions
      match instruction {
        // basic Brainfuck instruction set
        '>' => self.bp += 1,
        '<' => self.bp -= 1,
        '+' => self.block[self.bp] += 1,
        '-' => self.block[self.bp] -= 1,

        '[' => {
          self.loops[self.lp] = self.ip;
          self.lp += 1;
        }

        ']' => {
          if self.lp == 0 {
            println!("Error: tried to close loop while not inside any.");
            return;
          }
          if self.block[self.bp] != 0 {
            self.ip = self.loops[self.lp - 1];
          } else {
            self.lp -= 1;
          }
        }

        '.' => print!(
          "{}",
          std::char::from_u32(self.block[self.bp] as u32).unwrap_or('$')
        ),
        
        ',' => {
          let chr = std::io::stdin().bytes().next().unwrap().unwrap();
          self.block[self.bp] = chr as i32;
        },

        // custom macros / extensions (ignored by normal bf)
        '#' => self.display_block(0, 10),
        
        _ => {}
        
      }
      
      self.ip += 1;
    }
  }
  
  /// Print current block values in interval <from, to>
  pub fn display_block(&self, from: usize, to: usize) {
    let mut ss = String::from(format!("# {}=[", from));
    for i in from..=to {
      ss.push_str(&if i == self.bp {
        format!("({})", self.block[i])
      } else {
        format!("{}", self.block[i])
      });
      if i != to {
        ss.push_str(", ")
      }
    }
    ss.push_str(&format!("]={}", to));
    println!("{}", ss);
  }
}
