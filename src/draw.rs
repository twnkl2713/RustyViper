use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;
use piston_window::{text, Glyphs};
use piston_window::Transformed;

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}

#[allow(dead_code)]
pub fn draw_text_with_bg(
    text_str: &str,
    font_size: u32,
    text_color: Color,
    bg_color: Option<Color>,
    x: f64,
    y: f64,
    con: &Context,
    g: &mut G2d,
    glyphs: &mut Glyphs,
) {
    let padding = 10.0;
    if let Some(bg) = bg_color {
        let text_width = text_str.len() as f64 * 10.0; 
        rectangle(
            bg,
            [x - padding / 2.0, y - font_size as f64 + 5.0, text_width + padding, font_size as f64 + 10.0],
            con.transform,
            g,
        );
    }

    text::Text::new_color(text_color, font_size)
        .draw(text_str, glyphs, &con.draw_state, con.transform.trans(x, y), g)
        .unwrap();
}

#[allow(dead_code)]
pub fn draw_game_ui(
    score: usize,
    level: usize,
    con: &Context,
    g: &mut G2d,
    glyphs: &mut Glyphs,
) {
    let score_text = format!("Score: {}", score);
    let level_text = format!("Level: {}", level);

    text::Text::new_color([1.0, 1.0, 1.0, 1.0], 20)
        .draw(&score_text, glyphs, &con.draw_state, con.transform.trans(10.0, 25.0), g)
        .ok();

    text::Text::new_color([1.0, 1.0, 1.0, 1.0], 32)
        .draw(&level_text, glyphs, &con.draw_state, con.transform.trans(10.0, 45.0), g)
        .ok();
}

#[allow(dead_code)]
pub fn draw_centered_text(
    text_str: &str,
    font_size: u32,
    text_color: Color,
    x_center: f64,
    y: f64,
    con: &Context,
    g: &mut G2d,
    glyphs: &mut Glyphs,
) {
    let approx_text_width = text_str.len() as f64 * (font_size as f64 * 0.6); // rough estimate
    let x = x_center - approx_text_width / 2.0;

    text::Text::new_color(text_color, font_size)
        .draw(text_str, glyphs, &con.draw_state, con.transform.trans(x, y), g)
        .ok();
}
