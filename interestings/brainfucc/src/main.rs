
mod bf_engine;

use bf_engine::BrainfuckEngine;

fn main() {
  let eng = BrainfuckEngine::new();
  eng.display_block(0, 10);
}