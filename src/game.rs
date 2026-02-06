use crate::{
    board::{BOARD, BOARD_HEIGHT, BOARD_WIDTH, empty_board, render_board},
    piece::PlayerPiece,
};
use macroquad::prelude::*;

pub struct Game {
    pub board: BOARD,
    pub player_piece: PlayerPiece,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: empty_board(),
            player_piece: PlayerPiece::new_random_piece(),
        }
    }

    pub fn render(&self) {
        let mut board_with_player: BOARD = self.board;
        for dr in 0..4 {
            for dc in 0..4 {
                // Note that the position of the player can actually be off the grid
                if (0..BOARD_HEIGHT as isize).contains(&(self.player_piece.y + dr))
                    && (0..BOARD_WIDTH as isize).contains(&(self.player_piece.x + dc))
                {
                    board_with_player[(self.player_piece.y + dr) as usize]
                        [(self.player_piece.x + dc) as usize] +=
                        self.player_piece.piece_grid[dr as usize][dc as usize];
                }
            }
        }
        render_board(board_with_player);
    }

    pub fn move_player_left(&mut self) {
        if self.player_piece.x + (self.player_piece.min_x() as isize) > 0 {
            self.player_piece.x -= 1;
        }
    }
    pub fn move_player_right(&mut self) {
        if self.player_piece.x + (self.player_piece.max_x() as isize) < BOARD_WIDTH as isize - 1 {
            self.player_piece.x += 1;
        }
    }
    // TODO: remove this one lol
    pub fn move_player_up(&mut self) {
        if self.player_piece.y + (self.player_piece.min_y() as isize) > 0 {
            self.player_piece.y -= 1;
        }
    }
    pub fn move_player_down(&mut self) {
        if self.player_piece.y + (self.player_piece.max_y() as isize) < BOARD_HEIGHT as isize - 1 {
            self.player_piece.y += 1;
        }
    }

    pub fn rotate_player(&mut self) {}
}
