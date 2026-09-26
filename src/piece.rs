use std::collections::HashMap;

enum PieceKind {
    Pawn   = 0,
    Knight = 1,
    Bishop = 2,
    Rook   = 3,
    Queen  = 4,
    King   = 5
}

impl Into<usize> for PieceKind {
    fn into(self) -> usize {
        self as usize
    }
}

const PIECE_VALUES: [f32; 6] = [1.0, 3.0, 3.0, 5.0, 9.0, f32::INFINITY];
const PAIRED_BISHOP_VALUE: f32 = 3.5;


