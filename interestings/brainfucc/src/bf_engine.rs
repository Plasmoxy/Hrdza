
pub const BLOCK_SIZE: usize = 30000;

#[derive(PartialEq)]
pub enum EntryType {
  None,
  Number,
  String,
}

pub struct BrainfuckEngine {
  block: [i32; BLOCK_SIZE], // block of i32 (not bytes, i want more power)
  ip: usize, // instruction pointer in source file
  bp: usize, // block pointer inside block (current cell index)
  loops: [usize; 255], // array of nested loop pointers
  lp: usize, // current loop is lp-1, lp==0 means no current loop
}

impl BrainfuckEngine {
  pub fn new() -> BrainfuckEngine {
    BrainfuckEngine {
      block: [0; BLOCK_SIZE],
      ip: 0,
      bp: 0,
      loops: [0; 255],
      lp: 0,
    }
  }
  
  /// Print current block values in interval <from, to>
  pub fn display_block(&self, from: usize, to: usize) {
    let mut ss = String::from(format!("# {}=[", from));
    for i in from..=to {
        ss.push_str(& if i == self.bp {
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

