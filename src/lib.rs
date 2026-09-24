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

    .--- 0th Index            LERF Representation           63rd Index ---.
    v                                                                     v
    00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
    ^                                                                     ^
    `--- 63rd Index         Standard Representation          0th Index ---`
*/


pub struct Bitboard(pub u64);

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for i in 1..=64 { 
            str.push_str(&format!("{}  ", self.test(i - 1) as i32)); 
            if i % 8 == 0 {
                str.push('\n');
            }
        }
        write!(f, "{str}")
    }
}

impl Bitboard {
    pub fn test(&self, idx: u8) -> bool {
        self.0 & (1 << 63 - idx) != 0
    } 
}
