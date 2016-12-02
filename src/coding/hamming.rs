//! Encoding and decoding of the (15, 11, 3) standard and (10, 6, 3) shortened Hamming
//! codes described by P25.
//!
//! Both codes can correct up to 1 error. These algorithms are sourced from *Coding Theory
//! and Cryptography: The Essentials*, Hankerson, Hoffman, et al, 2000.

/// Encoding and decoding of the (15, 11, 3) code.
pub mod standard {
    use super::HammingDecoder;

    /// Encode the given 11 bits of data into a 15-bit codeword.
    pub fn encode(data: u16) -> u16 {
        assert!(data >> 11 == 0);
        matrix_mul_systematic!(data, GEN, u16)
    }

    /// Try to decode the given 15-bit word to the nearest codeword, correcting up to 1
    /// error.
    ///
    /// If decoding was successful, return `Some((data, err))`, where `data` is the 11
    /// data bits and `err` is the number of corrected bits. Otherwise, return `None` to
    /// indicate an unrecoverable error.
    pub fn decode(word: u16) -> Option<(u16, usize)> {
        assert!(word >> 15 == 0);
        StandardHamming::decode(word)
    }

    /// Generator patterns for 4 parity bits.
    const GEN: [u16; 4] = [
        0b11111110000,
        0b11110001110,
        0b11001101101,
        0b10101011011,
    ];

    /// Parity-check patterns for 4 syndromes.
    const PAR: [u16; 4] = [
        0b111111100001000,
        0b111100011100100,
        0b110011011010010,
        0b101010110110001,
    ];

    /// Maps 4-bit syndrome values to bit error locations.
    const LOCATIONS: [u16; 16] = [
        0,
        0b0000000000000001,
        0b0000000000000010,
        0b0000000000010000,
        0b0000000000000100,
        0b0000000000100000,
        0b0000000001000000,
        0b0000000010000000,
        0b0000000000001000,
        0b0000000100000000,
        0b0000001000000000,
        0b0000010000000000,
        0b0000100000000000,
        0b0001000000000000,
        0b0010000000000000,
        0b0100000000000000,
    ];

    struct StandardHamming;

    impl super::HammingDecoder for StandardHamming {
        type Data = u16;

        fn data(word: u16) -> u16 { word >> 4 }
        fn par() -> [u16; 4] { PAR }
        fn locs() -> [u16; 16] { LOCATIONS }
    }
}

/// Encoding and decoding of the (10, 6, 3) code.
pub mod shortened {
    use super::HammingDecoder;

    /// Encode the given 6 data bits into a 10-bit codeword.
    pub fn encode(data: u8) -> u16 {
        assert!(data >> 6 == 0);
        matrix_mul_systematic!(data, GEN, u16)
    }

    /// Try to decode the given 10-bit word to the nearest codeword, correcting up to 1
    /// error.
    ///
    /// If decoding was successful, return `Some((data, err))`, where `data` is the 6
    /// data bits and `err` is the number of corrected bits. Otherwise, return `None` to
    /// indicate an unrecoverable error.
    pub fn decode(word: u16) -> Option<(u8, usize)> {
        assert!(word >> 10 == 0);
        ShortHamming::decode(word)
    }

    const GEN: [u8; 4] = [
        0b111001,
        0b110101,
        0b101110,
        0b011110,
    ];

    const PAR: [u16; 4] = [
        0b1110011000,
        0b1101010100,
        0b1011100010,
        0b0111100001,
    ];

    const LOCATIONS: [u16; 16] = [
        0,
        0b0000000000000001,
        0b0000000000000010,
        0b0000000000100000,
        0b0000000000000100,
        0,
        0,
        0b0000000001000000,
        0b0000000000001000,
        0,
        0,
        0b0000000010000000,
        0b0000000000010000,
        0b0000000100000000,
        0b0000001000000000,
        0,
    ];

    struct ShortHamming;

    impl super::HammingDecoder for ShortHamming {
        type Data = u8;

        fn data(word: u16) -> u8 { (word >> 4) as u8 }
        fn par() -> [u16; 4] { PAR }
        fn locs() -> [u16; 16] { LOCATIONS }
    }
}

/// Defines code-specific decoding functions.
trait HammingDecoder {
    /// The type of the data bit output.
    type Data;

    /// Convert the codeword to data bits.
    fn data(word: u16) -> Self::Data;

    /// Return the parity-check patterns for 4 syndromes.
    fn par() -> [u16; 4];

    /// Return the syndrome-error location map.
    fn locs() -> [u16; 16];

    /// Use the current decoder to decode the given word.
    fn decode(word: u16) -> Option<(Self::Data, usize)> {
        // Compute the 4-bit syndrome.
        let s = matrix_mul!(word, Self::par(), u8);

        // A zero syndrome means it's a valid codeword (possibly different from the
        // transmitted codeword.)
        if s == 0 {
            return Some((Self::data(word), 0));
        }

        match Self::locs().get(s as usize) {
            // More than one error/unrecoverable error.
            Some(&0) | None => None,
            // Valid location means the error can be corrected.
            Some(&loc) => Some((Self::data(word ^ loc), 1)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_standard() {
        let w = 0b10101010101;
        let e = standard::encode(w);
        assert_eq!(standard::decode(e^0b000000000000000).unwrap().0, w);
        assert_eq!(standard::decode(e^0b000000000000001).unwrap().0, w);
        assert_eq!(standard::decode(e^0b000000000000010).unwrap().0, w);
        assert_eq!(standard::decode(e^0b000000000000100).unwrap().0, w);
        assert_eq!(standard::decode(e^0b000000000001000).unwrap().0, w);
        assert_eq!(standard::decode(e^0b000000000010000).unwrap().0, w);
        assert_eq!(standard::decode(e^0b000000000100000).unwrap().0, w);
        assert_eq!(standard::decode(e^0b000000001000000).unwrap().0, w);
        assert_eq!(standard::decode(e^0b000000010000000).unwrap().0, w);
        assert_eq!(standard::decode(e^0b000000100000000).unwrap().0, w);
        assert_eq!(standard::decode(e^0b000001000000000).unwrap().0, w);
        assert_eq!(standard::decode(e^0b000010000000000).unwrap().0, w);
        assert_eq!(standard::decode(e^0b000100000000000).unwrap().0, w);
        assert_eq!(standard::decode(e^0b001000000000000).unwrap().0, w);
        assert_eq!(standard::decode(e^0b010000000000000).unwrap().0, w);
        assert_eq!(standard::decode(e^0b100000000000000).unwrap().0, w);

        for i in 0..1<<11 {
            assert_eq!(standard::decode(standard::encode(i)).unwrap().0, i);
        }
    }

    #[test]
    fn test_shortened() {
        let w = 0b110011;
        let e = shortened::encode(w);
        assert_eq!(shortened::decode(e ^ 0b0000000000).unwrap().0, w);
        assert_eq!(shortened::decode(e ^ 0b0000000001).unwrap().0, w);
        assert_eq!(shortened::decode(e ^ 0b0000000010).unwrap().0, w);
        assert_eq!(shortened::decode(e ^ 0b0000000100).unwrap().0, w);
        assert_eq!(shortened::decode(e ^ 0b0000001000).unwrap().0, w);
        assert_eq!(shortened::decode(e ^ 0b0000010000).unwrap().0, w);
        assert_eq!(shortened::decode(e ^ 0b0000100000).unwrap().0, w);
        assert_eq!(shortened::decode(e ^ 0b0001000000).unwrap().0, w);
        assert_eq!(shortened::decode(e ^ 0b0010000000).unwrap().0, w);
        assert_eq!(shortened::decode(e ^ 0b0100000000).unwrap().0, w);
        assert_eq!(shortened::decode(e ^ 0b1000000000).unwrap().0, w);

        for i in 0..1<<6 {
            assert_eq!(shortened::decode(shortened::encode(i)).unwrap().0, i);
        }
    }
}
