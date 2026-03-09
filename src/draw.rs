use piston_window::graphics::{
    Context, Graphics, rectangle,
    types::{Color, Rectangle},
};

/// Size of a single game block in pixels.
const BLOCK_SIZE: f64 = 25.0;

/// Converts a grid coordinate to a pixel coordinate.
pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

/// Draws a single 1×1 block at the given grid position.
pub fn draw_block<G: Graphics>(color: Color, x: i32, y: i32, con: &Context, g: &mut G) {
    draw_rectangle(color, x, y, 1, 1, con, g);
}

/// Draws a filled rectangle spanning multiple grid cells.
pub fn draw_rectangle<G: Graphics>(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G,
) {
    let rect: Rectangle = [
        to_coord(x),
        to_coord(y),
        BLOCK_SIZE * (width as f64),
        BLOCK_SIZE * (height as f64),
    ];

    rectangle(color, rect, con.transform, g);
}
