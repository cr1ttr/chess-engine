/*
    ####################
    ### LERF-Mapping ###
    ####################
    
    Resource: https://chessprogramming.org/Square_Mapping_Considerations#little-endian-file-rank-mapping

    #################################
    ### Board Form Representation ###
    #################################

    +=========================+
    | 56 57 58 59 60 61 62 63 |
    | 48 49 50 51 52 53 54 55 | 
    | 40 41 42 43 44 45 46 47 |
    | 32 33 34 35 36 37 38 39 |
    | 24 25 26 27 28 29 30 31 | 
    | 16 17 18 19 20 21 22 23 | 
    | 08 09 10 11 12 13 14 15 | 
    | 00 01 02 03 04 05 06 07 |
    +=========================+

    // 1 9 17 25 33 41 49 57 
    // 0000000100000010000000010000001000000001000000100000000100000010

    ##################################
    ### String Form Representation ###
    ##################################

    00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
    ^                                                                     ^
    `--- 63rd Index         Standard Representation          0th Index ---`
*/

pub struct Bitboard(pub u64);

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();

        let mut idx: u8 = 56;

        for i in 1..=64 {
            str.push_str(&format!("{}  ", self.test(idx) as i32)); 
            if i % 8 == 0 && idx != 7 {
                idx -= 15;
                str.push('\n');
            } else {
                idx += 1;
            } 
        }
        write!(f, "{str}")
    }
}

impl Bitboard {
    pub fn test(&self, idx: u8) -> bool {
        self.0 & (1 << idx) != 0
    } 
}

pub enum Direction {
    NorthWest = 7,
    North = 8,
    NorthEast = 9, 
    West = -1, 
    None = 0,
    East = 1,
    SouthWest = -9,
    South = -8, 
    SouthEast = -7
}

pub enum Square {
    A1, A2, A3, A4, A5, A6, A7, A8,
    B1, B2, B3, B4, B5, B6, B7, B8,
    C1, C2, C3, C4, C5, C6, C7, C8,
    D1, D2, D3, D4, D5, D6, D7, D8,
    E1, E2, E3, E4, E5, E6, E7, E8, 
    F1, F2, F3, F4, F5, F6, F7, F8,
    G1, G2, G3, G4, G5, G6, G7, G8, 
    H1, H2, H3, H4, H5, H6, H7, H8
}

pub const FILE_A_MASK: Bitboard = Bitboard(0x0101010101010101);
pub const FILE_B_MASK: Bitboard = Bitboard(0x0202020202020202);
pub const FILE_C_MASK: Bitboard = Bitboard(0x0404040404040404);
pub const FILE_D_MASK: Bitboard = Bitboard(0x0808080808080808);
pub const FILE_E_MASK: Bitboard = Bitboard(0x1010101010101010);
pub const FILE_F_MASK: Bitboard = Bitboard(0x2020202020202020);
pub const FILE_G_MASK: Bitboard = Bitboard(0x4040404040404040);
pub const FILE_H_MASK: Bitboard = Bitboard(0x8080808080808080);

pub const RANK_1_MASK: Bitboard = Bitboard(0x00000000000000FF);
pub const RANK_2_MASK: Bitboard = Bitboard(0x000000000000FF00);
pub const RANK_3_MASK: Bitboard = Bitboard(0x0000000000FF0000);
pub const RANK_4_MASK: Bitboard = Bitboard(0x00000000FF000000);
pub const RANK_5_MASK: Bitboard = Bitboard(0x000000FF00000000);
pub const RANK_6_MASK: Bitboard = Bitboard(0x0000FF0000000000);
pub const RANK_7_MASK: Bitboard = Bitboard(0x00FF000000000000);
pub const RANK_8_MASK: Bitboard = Bitboard(0xFF00000000000000);

pub const MAIN_DIAGONAL_MASK: Bitboard = Bitboard(0x8040201008040201);
pub const ANTI_DIAGONAL_MASK: Bitboard = Bitboard(0x0102040810204080);
pub const LIGHT_SQUARES_MASK: Bitboard = Bitboard(0x55AA55AA55AA55AA);
pub const DARK_SQUARES_MASK: Bitboard = Bitboard(0xAA55AA55AA55AA55);