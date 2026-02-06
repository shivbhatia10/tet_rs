use macroquad::prelude::Color;

pub type PieceGrid = [[bool; 4]; 4];

pub const PIECE_GRIDS: [PieceGrid; 7] = [
    // O
    [
        [false, false, false, false],
        [false, true, true, false],
        [false, true, true, false],
        [false, false, false, false],
    ],
    // I
    [
        [false, true, false, false],
        [false, true, false, false],
        [false, true, false, false],
        [false, true, false, false],
    ],
    // L
    [
        [false, false, false, false],
        [false, true, false, false],
        [false, true, false, false],
        [false, true, true, false],
    ],
    // S
    [
        [false, false, false, false],
        [false, false, true, true],
        [false, true, true, false],
        [false, false, false, false],
    ],
    // Z
    [
        [false, false, false, false],
        [false, true, true, false],
        [false, false, true, true],
        [false, false, false, false],
    ],
    // J
    [
        [false, false, false, false],
        [false, false, true, false],
        [false, false, true, false],
        [false, true, true, false],
    ],
    // T
    [
        [false, false, false, false],
        [false, true, true, true],
        [false, false, true, false],
        [false, false, false, false],
    ],
];

#[derive(Clone)]
pub struct PlayerPiece {
    pub piece_grid: PieceGrid,
    pub color: Color,
    pub x: isize,
    pub y: isize,
}

impl PlayerPiece {
    pub fn new_random_piece(piece_index: usize, color: Color) -> Self {
        PlayerPiece {
            piece_grid: PIECE_GRIDS[piece_index],
            color,
            x: 3,
            y: 0,
        }
    }
}

pub fn rotate_piece_grid_clockwise(grid: PieceGrid) -> PieceGrid {
    let mut out = [[false; 4]; 4];
    for r in 0..4 {
        for c in 0..4 {
            out[c][3 - r] = grid[r][c];
        }
    }
    out
}
