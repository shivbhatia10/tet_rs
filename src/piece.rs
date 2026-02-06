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

#[derive(Clone)]
pub struct PlayerPiece {
    pub piece_grid: PieceGrid,
    // Position of top left corner
    // Note that these can be outside the grid.
    pub x: isize,
    pub y: isize,
}

impl PlayerPiece {
    pub fn new_random_piece(piece_index: usize, color: usize) -> Self {
        let base_grid = PIECE_GRIDS[piece_index];
        let mut piece_grid = [[0usize; 4]; 4];
        for r in 0..4 {
            for c in 0..4 {
                piece_grid[r][c] = base_grid[r][c] * color;
            }
        }
        PlayerPiece {
            piece_grid,
            x: 3,
            y: 0,
        }
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
