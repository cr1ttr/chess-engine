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


pub mod pawn {
    use crate::{Bitboard, Direction};
    use std::ops::BitXor;

    pub fn fill_black_pawn_front_spans(generators: Bitboard) -> Bitboard {
        Bitboard::fill_dir(black_pawn_stop_squares(generators), Bitboard::UNIVERSE_SET, Direction::South)
    }
    
    pub fn fill_black_rear_pawn_spans(generators: Bitboard) -> Bitboard {
        Bitboard::fill_dir(Bitboard::step_dir(generators, Direction::North), Bitboard::UNIVERSE_SET, Direction::North)
    }
    
    pub fn fill_white_pawn_front_spans(generators: Bitboard) -> Bitboard {
        Bitboard::fill_dir(white_pawn_stop_squares(generators), Bitboard::UNIVERSE_SET, Direction::North)
    }
    
    pub fn fill_white_pawn_rear_spans(generators: Bitboard) -> Bitboard {
        Bitboard::fill_dir(Bitboard::step_dir(generators, Direction::South), Bitboard::UNIVERSE_SET, Direction::South)
    }
    
    pub fn fill_pawn_interspans(black_generators: Bitboard, white_generators: Bitboard) -> Bitboard {
        fill_black_pawn_front_spans(black_generators) & fill_white_pawn_front_spans(white_generators) 
    }
    
    pub fn white_pawn_stop_squares(generators: Bitboard) -> Bitboard {
        Bitboard::step_dir(generators, Direction::North)
    }
    
    pub fn black_pawn_stop_squares(generators: Bitboard) -> Bitboard {
        Bitboard::step_dir(generators, Direction::South)
    }
    
    pub fn white_pawn_telestops(generators: Bitboard) -> Bitboard {
        fill_white_pawn_front_spans(generators).bitxor(white_pawn_stop_squares(generators))
    }
    
    pub fn black_pawn_telestops(generators: Bitboard) -> Bitboard {
        fill_black_pawn_front_spans(generators).bitxor(black_pawn_stop_squares(generators))
    }
}
    
