use macroquad::prelude::*;

use crate::piece::PlayerPiece;
pub const BOARD_HEIGHT: usize = 40;
pub const BOARD_WIDTH: usize = 10;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum Cell {
    #[default]
    Empty,
    Filled(Color),
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
        Cell::Filled(color) => color,
    }
}

pub fn apply_player_to_board(player: &PlayerPiece, board: &Board) -> Board {
    apply_player_to_board_ghost(player, board, 1.0)
}

pub fn apply_player_to_board_ghost(player: &PlayerPiece, board: &Board, intensity: f32) -> Board {
    let mut board_with_player: Board = *board;
    for dr in 0..4 {
        for dc in 0..4 {
            if !player.piece_grid[dr as usize][dc as usize] {
                continue;
            }
            let r = player.y + dr;
            let c = player.x + dc;
            if (0..BOARD_HEIGHT as isize).contains(&r) && (0..BOARD_WIDTH as isize).contains(&c) {
                let bg = cell_to_color(board_with_player[r as usize][c as usize]);
                let ghost_color = Color::new(
                    player.color.r * intensity + bg.r * (1.0 - intensity),
                    player.color.g * intensity + bg.g * (1.0 - intensity),
                    player.color.b * intensity + bg.b * (1.0 - intensity),
                    1.0,
                );
                board_with_player[r as usize][c as usize] = Cell::Filled(ghost_color);
            }
        }
    }
    board_with_player
}
