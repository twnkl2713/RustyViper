extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use crate::draw::to_coord_u32;
use crate::game::Game;

const BACK_COLOR: Color = [0.2, 0.2, 0.2, 1.0]; // Dark background

fn main() {
    let (w, h) = (30, 30);

    let mut window: PistonWindow = WindowSettings::new(
        "Snake++",
        [to_coord_u32(w), to_coord_u32(h)],
    )
    .exit_on_esc(true)
    .resizable(false)
    .build()
    .expect("Failed to create window");

    let mut glyphs = window
        .load_font("FiraSans-Regular.ttf")
        .expect("Font loading failed. Place 'FiraSans-Regular.ttf' in root if text is used.");

    let mut game = Game::new(w, h);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        event.update(|arg| {
            game.update(arg.dt);
        });

        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g, &mut glyphs);
        });
    }
}
