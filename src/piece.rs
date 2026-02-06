use std::cmp::{max, min};

type PieceGrid = [[usize; 4]; 4];

pub const PIECE_GRIDS: [PieceGrid; 7] = [
    // O
    [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
    // I
    [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]],
    // L
    [[0, 0, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0]],
    // S
    [[0, 0, 0, 0], [0, 0, 1, 1], [0, 1, 1, 0], [0, 0, 0, 0]],
    // Z
    [[0, 0, 0, 0], [0, 1, 1, 0], [0, 0, 1, 1], [0, 0, 0, 0]],
    // J
    [[0, 0, 0, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 1, 1, 0]],
    // T
    [[0, 0, 0, 0], [0, 1, 1, 1], [0, 0, 1, 0], [0, 0, 0, 0]],
];

pub struct PlayerPiece {
    pub piece_grid: PieceGrid,
    // Position of top left corner
    // Note that these can be outside the grid.
    pub x: isize,
    pub y: isize,
}

impl PlayerPiece {
    pub fn new_random_piece() -> Self {
        // let piece_grid_index = gen_range(0, 6) as usize;
        PlayerPiece {
            piece_grid: PIECE_GRIDS[5],
            x: 3,
            y: 0,
        }
    }

    /// The smallest non-zero x value in the local piece grid.
    pub fn min_x(&self) -> usize {
        let mut min_x: usize = 5;
        for row in 0..4 {
            for col in 0..4 {
                if self.piece_grid[row][col] > 0 {
                    min_x = min(min_x, col as usize);
                }
            }
        }
        min_x
    }

    /// The largest non-zero x value in the local piece grid.
    pub fn max_x(&self) -> usize {
        let mut max_x: usize = 0;
        for row in 0..4 {
            for col in 0..4 {
                if self.piece_grid[row][col] > 0 {
                    max_x = max(max_x, col as usize);
                }
            }
        }
        max_x
    }

    /// The smallest non-zero y value in the local piece grid.
    pub fn min_y(&self) -> usize {
        let mut min_y: usize = 5;
        for row in 0..4 {
            for col in 0..4 {
                if self.piece_grid[row][col] > 0 {
                    min_y = min(min_y, row as usize);
                }
            }
        }
        min_y
    }

    /// The largest non-zero y value in the local piece grid.
    pub fn max_y(&self) -> usize {
        let mut max_y: usize = 0;
        for row in 0..4 {
            for col in 0..4 {
                if self.piece_grid[row][col] > 0 {
                    max_y = max(max_y, row as usize);
                }
            }
        }
        max_y
    }
}

pub fn rotate_piece_grid_clockwise(grid: PieceGrid) -> PieceGrid {
    let mut out = [[0usize; 4]; 4];
    for r in 0..4 {
        for c in 0..4 {
            out[c][3 - r] = grid[r][c];
        }
    }
    out
}
