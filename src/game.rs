use crate::{
    board::{BOARD, BOARD_HEIGHT, BOARD_WIDTH, empty_board, render_board},
    piece::{PlayerPiece, rotate_piece_grid_clockwise},
};
use macroquad::rand::{gen_range, srand};
use macroquad::{miniquad::date::now, prelude::*};

pub struct Game {
    pub board: BOARD,
    pub player_piece: PlayerPiece,
    pub score: usize,
}

const WALL_KICKS: [[isize; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

impl Game {
    pub fn new() -> Self {
        srand(now() as u64);
        Game {
            board: empty_board(),
            player_piece: Self::spawn_piece(),
            score: 0,
        }
    }

    fn spawn_piece() -> PlayerPiece {
        let piece_index = gen_range(0, 7) as usize;
        let color = gen_range(1, 8) as usize;
        PlayerPiece::new_random_piece(piece_index, color)
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
        self.player_piece.x -= 1;
        if self.has_collision() {
            self.player_piece.x += 1;
        }
    }
    pub fn move_player_right(&mut self) {
        self.player_piece.x += 1;
        if self.has_collision() {
            self.player_piece.x -= 1;
        }
    }
    pub fn move_player_down(&mut self) {
        self.player_piece.y += 1;
        if self.has_collision() {
            self.player_piece.y -= 1;
        }
    }

    pub fn rotate_player(&mut self) {
        self.player_piece.piece_grid = rotate_piece_grid_clockwise(self.player_piece.piece_grid);
        if !self.has_collision() {
            return;
        }
        for [dx, dy] in WALL_KICKS {
            self.player_piece.x += dx;
            self.player_piece.y += dy;
            if !self.has_collision() {
                return;
            }
            self.player_piece.x -= dx;
            self.player_piece.y -= dy;
        }
        panic!("Unfixable position")
    }

    fn has_collision(&self) -> bool {
        self.player_piece
            .piece_grid
            .iter()
            .enumerate()
            .any(|(dr, row)| {
                row.iter().enumerate().any(|(dc, val)| {
                    if val == &0 {
                        return false;
                    }
                    let r = self.player_piece.y + dr as isize;
                    let c = self.player_piece.x + dc as isize;
                    if r < 0 || r >= BOARD_HEIGHT as isize || c < 0 || c >= BOARD_WIDTH as isize {
                        return true;
                    }
                    self.board[r as usize][c as usize] > 0
                })
            })
    }

    pub fn check_if_piece_has_terminated(&mut self) {
        self.player_piece.y += 1;
        let collided = self.has_collision();
        self.player_piece.y -= 1;
        if collided {
            self.apply_player_to_board();
            self.player_piece = Self::spawn_piece();
            self.clear_rows_and_shift_squares_down();
        }
    }

    fn apply_player_to_board(&mut self) {
        for dr in 0..4 {
            for dc in 0..4 {
                // Note that the position of the player can actually be off the grid
                if (0..BOARD_HEIGHT as isize).contains(&(self.player_piece.y + dr))
                    && (0..BOARD_WIDTH as isize).contains(&(self.player_piece.x + dc))
                {
                    self.board[(self.player_piece.y + dr) as usize]
                        [(self.player_piece.x + dc) as usize] +=
                        self.player_piece.piece_grid[dr as usize][dc as usize];
                }
            }
        }
    }

    fn clear_rows_and_shift_squares_down(&mut self) {
        let mut new_board = empty_board();
        let mut new_y = BOARD_HEIGHT - 1;
        for old_y in (0..BOARD_HEIGHT).rev() {
            if self.board[old_y].iter().all(|&val| val > 0) {
                continue;
            }
            new_board[new_y] = self.board[old_y];
            if new_y == 0 {
                break;
            }
            new_y -= 1;
        }
        self.board = new_board;
        self.score += 1;
    }
}
