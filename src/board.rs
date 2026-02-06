use macroquad::prelude::*;
pub const BOARD_HEIGHT: usize = 40;
pub const BOARD_WIDTH: usize = 10;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum Cell {
    #[default]
    Empty,
    Filled(u8),
}

pub type Board = [[Cell; BOARD_WIDTH]; BOARD_HEIGHT];
pub fn square_width() -> f32 {
    screen_width() / (BOARD_WIDTH as f32)
}
pub fn square_height() -> f32 {
    screen_height() / (BOARD_HEIGHT as f32)
}

pub fn empty_board() -> Board {
    [[Cell::Empty; BOARD_WIDTH]; BOARD_HEIGHT]
}

pub fn render_board(board: Board) {
    for r in 0..BOARD_HEIGHT {
        for c in 0..BOARD_WIDTH {
            draw_rectangle(
                c as f32 * square_width(),
                r as f32 * square_height(),
                square_width(),
                square_height(),
                cell_to_color(board[r][c]),
            );
        }
    }
}

fn cell_to_color(cell: Cell) -> Color {
    match cell {
        Cell::Empty => WHITE,
        Cell::Filled(1) => RED,
        Cell::Filled(2) => GREEN,
        Cell::Filled(3) => BLUE,
        Cell::Filled(4) => GOLD,
        Cell::Filled(5) => ORANGE,
        Cell::Filled(6) => PURPLE,
        Cell::Filled(7) => MAGENTA,
        Cell::Filled(_) => DARKGRAY,
    }
}
