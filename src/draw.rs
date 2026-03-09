use piston_window::graphics::{
    Context, Graphics, rectangle,
    types::{Color, Rectangle},
    
};

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn draw_block<G: Graphics>(color: Color, x: i32, y: i32, con: &Context, g: &mut G) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    let rect: Rectangle = [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE];

    rectangle(color, rect, con.transform, g);
}

pub fn draw_rectangle<G: Graphics>(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G,
) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    let rect: Rectangle = [
        gui_x,
        gui_y,
        BLOCK_SIZE * (width as f64),
        BLOCK_SIZE * (height as f64),
    ];

    rectangle(color, rect, con.transform, g);
}
