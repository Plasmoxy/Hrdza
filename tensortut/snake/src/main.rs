#![windows_subsystem = "windows"]

mod draw;
mod game;
mod snake;

use draw::to_coord;
use game::Game;
use piston_window::types::Color;
use piston_window::*;

const BACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

fn main() {
    let (width, height) = (20, 20);
    let win_bounds = [to_coord(width) as u32, to_coord(height) as u32];

    let mut window: PistonWindow =
        WindowSettings::new("HYBAJUCI-ŠE KOKOT", win_bounds)
            .exit_on_esc(true)
            .build()
            .unwrap();
            
    let mut game = Game::new(width, height);
    
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });
        
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
