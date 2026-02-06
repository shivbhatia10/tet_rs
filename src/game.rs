use crate::{
    board::{
        BOARD_HEIGHT, BOARD_WIDTH, Board, Cell, apply_player_to_board, apply_player_to_board_ghost,
        empty_board, render_board,
    },
    piece::{PlayerPiece, rotate_piece_grid_clockwise},
};
use macroquad::rand::{gen_range, srand};
use macroquad::{miniquad::date::now, prelude::*};

pub struct Game {
    pub board: Board,
    pub player_piece: PlayerPiece,
    pub score: usize,
    pub game_over: bool,
    pub show_ghost: bool,
}

const WALL_KICKS: [[isize; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

impl Game {
    pub fn new(show_ghost: bool) -> Self {
        srand(now() as u64);
        Game {
            board: empty_board(),
            player_piece: Self::spawn_piece(),
            score: 0,
            game_over: false,
            show_ghost,
        }
    }

    const PIECE_COLORS: [Color; 7] = [RED, GREEN, BLUE, GOLD, ORANGE, PURPLE, MAGENTA];

    fn spawn_piece() -> PlayerPiece {
        let piece_index = gen_range(0, 7) as usize;
        let color = Self::PIECE_COLORS[gen_range(0, 7) as usize];
        PlayerPiece::new_piece(piece_index, color)
    }

    pub fn render(&self) {
        let mut board = apply_player_to_board(&self.player_piece, &self.board);

        if self.show_ghost {
            let ghost = self.get_ghost_player();
            board = apply_player_to_board_ghost(&ghost, &board, 0.3);
        }

        render_board(board);
    }

    pub fn get_ghost_player(&self) -> PlayerPiece {
        let mut ghost = self.player_piece.clone();
        while !self.player_piece_has_collision(&ghost) {
            ghost.y += 1;
        }
        ghost.y -= 1;
        ghost
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
    pub fn hard_drop(&mut self) {
        self.player_piece = self.get_ghost_player();
    }

    pub fn rotate_player(&mut self) {
        let old_grid = self.player_piece.piece_grid.clone();
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
        // This state seems impossible but if it happens, revert to old
        self.player_piece.piece_grid = old_grid;
    }

    fn has_collision(&self) -> bool {
        self.player_piece_has_collision(&self.player_piece)
    }

    fn player_piece_has_collision(&self, player_piece: &PlayerPiece) -> bool {
        player_piece.piece_grid.iter().enumerate().any(|(dr, row)| {
            row.iter().enumerate().any(|(dc, filled)| {
                if !filled {
                    return false;
                }
                let r = player_piece.y + dr as isize;
                let c = player_piece.x + dc as isize;
                if r < 0 || r >= BOARD_HEIGHT as isize || c < 0 || c >= BOARD_WIDTH as isize {
                    return true;
                }
                self.board[r as usize][c as usize] != Cell::Empty
            })
        })
    }

    pub fn check_if_piece_has_terminated(&mut self) {
        self.player_piece.y += 1;
        let collided = self.has_collision();
        self.player_piece.y -= 1;
        if collided {
            if self.player_piece.y == 0 {
                self.game_over = true;
            }
            self.board = apply_player_to_board(&self.player_piece, &self.board);
            self.player_piece = Self::spawn_piece();
            self.clear_rows_and_shift_squares_down();
        }
    }

    fn clear_rows_and_shift_squares_down(&mut self) {
        let mut new_board = empty_board();
        let mut new_y = BOARD_HEIGHT - 1;
        for old_y in (0..BOARD_HEIGHT).rev() {
            if self.board[old_y].iter().all(|&cell| cell != Cell::Empty) {
                // Increase score by one for each row cleared
                self.score += 1;
                continue;
            }
            new_board[new_y] = self.board[old_y];
            if new_y == 0 {
                break;
            }
            new_y -= 1;
        }
        self.board = new_board;
    }

    pub fn drop_interval(&self) -> f64 {
        0.2 + 0.8 * (-0.1 * self.score as f64).exp()
    }
}
