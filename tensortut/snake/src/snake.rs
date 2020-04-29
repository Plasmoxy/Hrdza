use crate::draw::{draw_block};
use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

const SNAKE_COLOR: Color = [0.0, 1.0, 0.862, 1.0];

// is partialeq derive required ?
// Copy -> Direction has now copy semantics
// PartialEq -> implement equality comparisons
// TODO: try disabling if it breaks shit
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {Up, Down, Left, Right}

impl Direction {
  pub fn opposite(&self) -> Direction {
    match *self {
      Direction::Up => Direction::Down,
      Direction::Down => Direction::Up,
      Direction::Left => Direction::Right,
      Direction::Right => Direction::Left,
    }
  }
}

#[derive(Debug, Clone)]
struct Block {
  x: i32,
  y: i32,
}

pub struct Snake {
  direction: Direction,
  body: LinkedList<Block>,
  tail: Option<Block>,
}

impl Snake {
  pub fn new(x: i32, y: i32) -> Snake {
    let mut body = LinkedList::new();
    
    // init body
    body.push_back(Block{ x: x + 2, y });
    body.push_back(Block{ x: x + 1, y });
    body.push_back(Block{ x, y });
    
    Snake {
      direction: Direction::Right,
      body,
      tail: None,
    }
  }
  
  pub fn draw(&self, con: &Context, g: &mut G2d) {
    for block in &self.body {
      draw_block(SNAKE_COLOR, block.x, block.y, con, g);
    }
  }
  
  pub fn head_position(&self) -> (i32, i32) {
    let head = self.body.front().unwrap();
    (head.x, head.y)
  }
  
  pub fn move_forward(&mut self, dir: Option<Direction>) {
    if let Some(d) = dir {
      self.direction = d;
    }
    
    // last xy
    let (lx, ly) = self.head_position();
    
    let new_block = match self.direction {
      Direction::Up    => Block { x: lx, y: ly - 1 },
      Direction::Down  => Block { x: lx, y: ly + 1 },
      Direction::Left  => Block { x: lx - 1, y: ly },
      Direction::Right => Block { x: lx + 1, y: ly },
    };
    
    self.body.push_front(new_block);
    let removed_block = self.body.pop_back().unwrap();
    self.tail = Some(removed_block);
  }
  
  pub fn head_direction(&self) -> Direction {
    self.direction.clone()
  }
  
  // TODO: use more idiomatic code
  pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
    let (hx, hy) = self.head_position();
    
    let mut moving_dir = self.direction; // copied
    
    if let Some(d) = dir {
      moving_dir = d;
    }
    
    match moving_dir {
      Direction::Up    => (hx, hy - 1),
      Direction::Down  => (hx, hy + 1),
      Direction::Left  => (hx - 1, hy),
      Direction::Right => (hx + 1, hy),
    }
  }
  
  pub fn restore_tail(&mut self) {
    self.body.push_back(
      self.tail.clone().unwrap()
    );
  }
  
  pub fn overlaps_tail(&self, x: i32, y: i32) -> bool {
    let mut ch = 0;
    
    for block in &self.body {
      if x == block.x && y == block.y {
        return true;
      }
      
      // wait wtf ???
      ch += 1;
      if ch == self.body.len() - 1 {
        break;
      }
    }
    
    false
  }
  
}