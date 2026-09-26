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

    ##################################
    ### String Form Representation ###
    ##################################

    00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
    ^                                                                     ^
    `--- 63rd Index         Standard Representation          0th Index ---`
*/

use std::ops::{Not, Shl, ShlAssign, Shr, ShrAssign};

use crate::{Square, position::Direction};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Bitboard(u64);

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        let mut idx: u8 = 56;

        str.push_str("\x1b[2m8  |  \x1b[0m");
        for i in 1..=64 {
            let bit_set = self.test(idx);

            let bit_str: &str = if bit_set {
                "\x1b[33m1\x1b[0m"
            } else {
                "\x1b[2m.\x1b[0m"
            };

            str.push_str(&format!("{bit_str}  ")); 
            if i % 8 == 0 && idx != 7 {
                idx -= 15;
                str.push_str(&format!("\x1b[2m\n{}  |\x1b[0m  ",  idx / 8 + 1));
            } else {
                idx += 1;
            } 
        }
        str.push_str("\n   \x1b[2m+ -----------------------\x1b[0m");
        str.push_str("\n      \x1b[2mA  B  C  D  E  F  G  H\x1b[0m");

        write!(f, "{str}")
    }
}

impl Bitboard {
    pub const UNIVERSE_SET: Self = Self(0xFFFFFFFFFFFFFFFF);
    pub const EMPTY_SET: Self = Self(0x0000000000000000);

    pub const FILE_A_MASK: Self = Self(0x0101010101010101);
    pub const FILE_B_MASK: Self = Self(0x0202020202020202);
    pub const FILE_C_MASK: Self = Self(0x0404040404040404);
    pub const FILE_D_MASK: Self = Self(0x0808080808080808);
    pub const FILE_E_MASK: Self = Self(0x1010101010101010);
    pub const FILE_F_MASK: Self = Self(0x2020202020202020);
    pub const FILE_G_MASK: Self = Self(0x4040404040404040);
    pub const FILE_H_MASK: Self = Self(0x8080808080808080);
    
    pub const RANK_1_MASK: Self = Self(0x00000000000000FF);
    pub const RANK_2_MASK: Self = Self(0x000000000000FF00);
    pub const RANK_3_MASK: Self = Self(0x0000000000FF0000);
    pub const RANK_4_MASK: Self = Self(0x00000000FF000000);
    pub const RANK_5_MASK: Self = Self(0x000000FF00000000);
    pub const RANK_6_MASK: Self = Self(0x0000FF0000000000);
    pub const RANK_7_MASK: Self = Self(0x00FF000000000000);
    pub const RANK_8_MASK: Self = Self(0xFF00000000000000);
    
    pub const MAIN_DIAGONAL_MASK: Self = Self(0x8040201008040201);
    pub const ANTI_DIAGONAL_MASK: Self = Self(0x0102040810204080);
    pub const LIGHT_SQUARES_MASK: Self = Self(0x55AA55AA55AA55AA);
    pub const DARK_SQUARES_MASK:  Self = Self(0xAA55AA55AA55AA55);

    const WRAP_MASK: [Self; 8] = [
        Self(0xFEFEFEFEFEFEFE00), // North East
        Self(0xFEFEFEFEFEFEFEFE), // East
        Self(0x00FEFEFEFEFEFEFE), // South East
        Self(0x00FFFFFFFFFFFFFF), // South
        Self(0x007F7F7F7F7F7F7F), // South West
        Self(0x7F7F7F7F7F7F7F7F), // West
        Self(0x7F7F7F7F7F7F7F00), // North West
        Self(0xFFFFFFFFFFFFFF00)  // North
    ];
    
    pub fn new(bits: u64) -> Self {
        Self(bits)
    }

    pub fn empty() -> Self {
        Bitboard::EMPTY_SET
    }

    pub fn test(&self, idx: u8) -> bool {
        self.0 & (1 << idx) != 0
    } 

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn is_populated(&self) -> bool {    
        self.0 != 0
    }

    pub fn population_count(&self) -> u8 {
        let mut count: u8 = 0;
        let mut tmp: u64 = self.0;

        while tmp != 0 {
            count += 1;
            tmp &= tmp - 1;
        }

        count
    }

    pub fn set(&mut self, idx: impl Into<u8>) -> Self {
        Self(self.0 | 1 << idx.into())
    }

    pub fn clear(&mut self, idx: impl Into<u8>) -> Self {
        Self(self.0 & !(1 << idx.into()))
    }

    pub fn toggle(&mut self, idx: impl Into<u8>) -> Self {
        Self(self.0 ^ 1 << idx.into())
    }

    pub fn fill_south(generators: Self, propagators: Self) -> Self {
        let mut generators = generators;
        let mut accumulator = Self::empty();

        let dir: u8 = Direction::South.into();

        generators = generators >> dir & propagators;
        accumulator |= generators;
        generators = generators >> dir & propagators;
        accumulator |= generators;
        generators = generators >> dir & propagators;
        accumulator |= generators;
        generators = generators >> dir & propagators;
        accumulator |= generators;
        generators = generators >> dir & propagators;
        accumulator |= generators;
        generators = generators >> dir & propagators;
        accumulator |= generators;
        generators = generators >> dir & propagators;
        accumulator |= generators;

        accumulator
    }

    pub fn fill_north(generators: Self, propagators: Self) -> Self {
        let mut generators = generators;
        let mut accumulator = Self::empty();

        let dir: u8 = Direction::North.into();

        generators = generators << dir & propagators;
        accumulator |= generators;
        generators = generators << dir & propagators;
        accumulator |= generators;
        generators = generators << dir & propagators;
        accumulator |= generators;
        generators = generators << dir & propagators;
        accumulator |= generators;
        generators = generators << dir & propagators;
        accumulator |= generators;
        generators = generators << dir & propagators;
        accumulator |= generators;
        generators = generators << dir & propagators;
        accumulator |= generators;

        accumulator
    }

    pub fn fill_east(generators: Self, propagators: Self) -> Self {
        let mut generators = generators;
        let mut accumulator = Self::empty();
        let propagators = propagators & !Bitboard::FILE_A_MASK;
        let dir: u8 = Direction::East.into();

        generators = generators << dir & propagators;
        accumulator |= generators;
        generators = generators << dir & propagators;
        accumulator |= generators;
        generators = generators << dir & propagators;
        accumulator |= generators;
        generators = generators << dir & propagators;
        accumulator |= generators;
        generators = generators << dir & propagators;
        accumulator |= generators;
        generators = generators << dir & propagators;
        accumulator |= generators;
        generators = generators << dir & propagators;
        accumulator |= generators;

        accumulator
    }

    pub fn fill_west(generators: Self, propagators: Self) -> Self {
        let mut generators = generators;
        let mut accumulator = Self::empty();
        let propagators = propagators & !Bitboard::FILE_H_MASK;
        let dir: u8 = Direction::West.into();

        generators = generators >> dir & propagators;
        accumulator |= generators;
        generators = generators >> dir & propagators;
        accumulator |= generators;
        generators = generators >> dir & propagators;
        accumulator |= generators;
        generators = generators >> dir & propagators;
        accumulator |= generators;
        generators = generators >> dir & propagators;
        accumulator |= generators;
        generators = generators >> dir & propagators;
        accumulator |= generators;
        generators = generators >> dir & propagators;
        accumulator |= generators;

        accumulator
    }

    pub fn fill_north_east(generators: Self, propagators: Self) -> Self {
        let mut generators = generators;
        let mut accumulator = Self::empty();
        let propagators = propagators & !Bitboard::FILE_A_MASK;
        let dir: u8 = Direction::NorthEast.into();

        generators = generators << dir & propagators;
        accumulator |= generators;
        generators = generators << dir & propagators;
        accumulator |= generators;
        generators = generators << dir & propagators;
        accumulator |= generators;
        generators = generators << dir & propagators;
        accumulator |= generators;
        generators = generators << dir & propagators;
        accumulator |= generators;
        generators = generators << dir & propagators;
        accumulator |= generators;
        generators = generators << dir & propagators;
        accumulator |= generators;

        accumulator
    }

    pub fn flip_vertical(&self) -> Self {
        (*self << 56) | 
        (*self << 40) & Bitboard::RANK_7_MASK | 
        (*self << 24) & Bitboard::RANK_6_MASK | 
        (*self << 8)  & Bitboard::RANK_5_MASK | 
        (*self >> 8)  & Bitboard::RANK_4_MASK | 
        (*self >> 24) & Bitboard::RANK_3_MASK | 
        (*self >> 40) & Bitboard::RANK_2_MASK | 
        (*self >> 56)
    }


    /*
        delta_swap_1:                delta_swap_2:                delta_swap_3:
        + ====================== +   + ====================== +   + ====================== +
        | 1  .  1  .  1  .  1  . |   | 1  1  .  1  1  .  .  . |   | 1  1  1  1  .  .  .  . |
        | 1  .  1  .  1  .  1  . |   | 1  1  .  1  1  .  .  . |   | 1  1  1  1  .  .  .  . |
        | 1  .  1  .  1  .  1  . |   | 1  1  .  1  1  .  .  . |   | 1  1  1  1  .  .  .  . |
        | 1  .  1  .  1  .  1  . |   | 1  1  .  1  1  .  .  . |   | 1  1  1  1  .  .  .  . |
        | 1  .  1  .  1  .  1  . |   | 1  1  .  1  1  .  .  . |   | 1  1  1  1  .  .  .  . |
        | 1  .  1  .  1  .  1  . |   | 1  1  .  1  1  .  .  . |   | 1  1  1  1  .  .  .  . |
        | 1  .  1  .  1  .  1  . |   | 1  1  .  1  1  .  .  . |   | 1  1  1  1  .  .  .  . |
        | 1  .  1  .  1  .  1  . |   | 1  1  .  1  1  .  .  . |   | 1  1  1  1  .  .  .  . |
        + ====================== +   + ====================== +   + ====================== +
    */

    pub fn flip_horizontal(&self) -> Self {
        let delta_swap_1 = Self::new(0x5555555555555555);
        let delta_swap_2 = Self::new(0x3333333333333333);
        let delta_swap_3 = Self::new(0x0f0f0f0f0f0f0f0f);

        let mut accumulator = self.clone();

        accumulator = ((accumulator >> 1) & delta_swap_1) | ((accumulator & delta_swap_1) << 1);
        accumulator = ((accumulator >> 2) & delta_swap_2) | ((accumulator & delta_swap_2) << 2);
        accumulator = ((accumulator >> 4) & delta_swap_3) | ((accumulator & delta_swap_3) << 4);

        accumulator
    }

    /*
        delta_swap_1                 delta_swap_2                 delta_swap_3
        + ====================== +   + ====================== +   + ====================== +
        | 1  .  1  .  1  .  1  . |   | 1  1  .  .  1  1  .  . |   | 1  1  1  1  .  .  .  . |
        | .  .  .  .  .  .  .  . |   | 1  1  .  .  1  1  .  . |   | 1  1  1  1  .  .  .  . |
        | 1  .  1  .  1  .  1  . |   | .  .  .  .  .  .  .  . |   | 1  1  1  1  .  .  .  . |
        | .  .  .  .  .  .  .  . |   | .  .  .  .  .  .  .  . |   | 1  1  1  1  .  .  .  . |
        | 1  .  1  .  1  .  1  . |   | 1  1  .  .  1  1  .  . |   | .  .  .  .  .  .  .  . |
        | .  .  .  .  .  .  .  . |   | 1  1  .  .  1  1  .  . |   | .  .  .  .  .  .  .  . |
        | 1  .  1  .  1  .  1  . |   | .  .  .  .  .  .  .  . |   | .  .  .  .  .  .  .  . |
        | .  .  .  .  .  .  .  . |   | .  .  .  .  .  .  .  . |   | .  .  .  .  .  .  .  . |
        + ====================== +   + ====================== +   + ====================== +
    */

    pub fn flip_main_diagonal(&self) -> Self {
        let mut tmp;
        let mut accumulator = self.clone();

        let delta_swap_1 = Bitboard::new(0x5500550055005500);
        let delta_swap_2 = Bitboard::new(0x3333000033330000);
        let delta_swap_3 = Bitboard::new(0x0f0f0f0f00000000);

        tmp = delta_swap_3 & (accumulator ^ (accumulator << 28));
        accumulator ^= tmp ^ (tmp >> 28);
        tmp = delta_swap_2 & (accumulator ^ (accumulator << 14));
        accumulator ^= tmp ^ (tmp >> 14);   
        tmp = delta_swap_1 & (accumulator ^ (accumulator << 7));
        accumulator ^= tmp ^ (tmp >> 7);

        accumulator
    }

     /*
    delta_swap_1                 delta_swap_2                 delta_swap_3
    + ====================== +   + ====================== +   + ====================== +
    | .  1  .  1  .  1  .  1 |   | .  .  1  1  .  .  1  1 |   | .  .  .  .  1  1  1  1 |
    | .  .  .  .  .  .  .  . |   | .  .  1  1  .  .  1  1 |   | .  .  .  .  1  1  1  1 |
    | .  1  .  1  .  1  .  1 |   | .  .  .  .  .  .  .  . |   | .  .  .  .  1  1  1  1 |
    | .  .  .  .  .  .  .  . |   | .  .  .  .  .  .  .  . |   | .  .  .  .  1  1  1  1 |
    | .  1  .  1  .  1  .  1 |   | .  .  1  1  .  .  1  1 |   | .  .  .  .  .  .  .  . |
    | .  .  .  .  .  .  .  . |   | .  .  1  1  .  .  1  1 |   | .  .  .  .  .  .  .  . |
    | .  1  .  1  .  1  .  1 |   | .  .  .  .  .  .  .  . |   | .  .  .  .  .  .  .  . |
    | .  .  .  .  .  .  .  . |   | .  .  .  .  .  .  .  . |   | .  .  .  .  .  .  .  . |
    + ====================== +   + ====================== +   + ====================== +
    */

    pub fn flip_anti_diagonal(&self) -> Self {
        let mut tmp;
        let mut accumulator = self.clone();

        let delta_swap_1 = Bitboard::new(0xaa00aa00aa00aa00);
        let delta_swap_2 = Bitboard::new(0xcccc0000cccc0000);
        let delta_swap_4 = Bitboard::new(0xf0f0f0f00f0f0f0f);

        tmp = accumulator ^ (accumulator << 36);
        accumulator ^= delta_swap_4 & (tmp ^ (accumulator >> 36));
        tmp = delta_swap_2 & (accumulator ^ (accumulator << 18));
        accumulator ^= tmp ^ (tmp >> 18);
        tmp = delta_swap_1 & (accumulator ^ (accumulator << 9));
        accumulator ^= tmp ^ (tmp >> 9);

        accumulator
    }

    pub fn rotate180(&self) -> Self {
        self.flip_horizontal().flip_vertical()
    }

    pub fn rotate90cw(&self) -> Self {
        self.flip_vertical().flip_anti_diagonal()
    }

    pub fn rotate90ccw(&self) -> Self {
        self.flip_vertical().flip_main_diagonal()
    }
}

impl std::ops::Add for Bitboard {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Bitboard {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for Bitboard {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Bitboard::EMPTY_SET
    }
}

impl std::ops::SubAssign for Bitboard {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0 
    }
}

impl std::ops::Mul for Bitboard {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0) 
    }
}

impl std::ops::MulAssign for Bitboard {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl std::ops::Div for Bitboard {
    type Output = Bitboard;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl std::ops::DivAssign for Bitboard {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl std::ops::BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl std::ops::BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}


impl Shl<u8> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl ShlAssign<u8> for Bitboard {
    fn shl_assign(&mut self, rhs: u8) {
        self.0 <<= rhs
    }
}

impl Shr<u8> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl ShrAssign<u8> for Bitboard {
    fn shr_assign(&mut self, rhs: u8) {
        self.0 >>= rhs
    }
}

impl std::ops::Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl From<Square> for Bitboard {
    fn from(value: Square) -> Self {
        let mut bb = Bitboard::empty();
        bb = bb.set(value);
        bb
    }
}


impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Self(value)
    }
}