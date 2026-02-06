use macroquad::prelude::*;
pub const BOARD_HEIGHT: usize = 40;
pub const BOARD_WIDTH: usize = 10;

pub type BOARD = [[usize; BOARD_WIDTH as usize]; BOARD_HEIGHT];
pub fn square_width() -> f32 {
    screen_width() / (BOARD_WIDTH as f32)
}
pub fn square_height() -> f32 {
    screen_height() / (BOARD_HEIGHT as f32)
}

pub fn empty_board() -> BOARD {
    [[0; BOARD_WIDTH]; BOARD_HEIGHT]
}

pub fn render_board(board: BOARD) {
    for r in 0..BOARD_HEIGHT {
        for c in 0..BOARD_WIDTH {
            draw_rectangle(
                c as f32 * square_width(),
                r as f32 * square_height(),
                square_width(),
                square_height(),
                val_to_color(board[r][c]),
            );
        }
    }
}

fn val_to_color(val: usize) -> Color {
    match val {
        0 => WHITE,
        1 => RED,
        2 => GREEN,
        3 => BLUE,
        4 => YELLOW,
        5 => ORANGE,
        6 => PURPLE,
        x => panic!("Invalid color val: {x}"),
    }
}
