//! Encode an index of a [nibble](https://en.wikipedia.org/wiki/Nibble) from a u64 as a u8.
//! 
//! `|      4 bits     |     4 bits     |`
//! 
//! `| index of nibble | nibble of data |`
//! 
//! # Usage
//! 
//! ```rust
//! use crumb::Crumb;
//! 
//! fn main() {
//!     let crumb = Crumb::new(1, 0b11111111).unwrap();
//!     // bits outside of the index are discarded
//!     assert_eq!(0b11110000u64, crumb.get_u64());
//! }
//! ```
//! 
//! ### Name
//! 
//! A nibble of something is about the size of a crumb or something like that...
//! 
//! ### Why?
//! 
//! Was doing some research on variable length integers and realized how nicely this fits.
//! 
//! ### For What?
//! 
//! Not sure!
//! 
//! I don't think there are too many applications for this.

#![no_std]

pub const MAX_NIBBLE_INDEX: u8 = 1u8 << 4;

/// 
#[derive(Default)]
pub struct Crumb (u8);

impl Crumb {
    /// constructs a crumb from an index and a source u64
    pub fn new(nibble_index: u8, src: u64) -> Option<Self> {
        if nibble_index >= MAX_NIBBLE_INDEX { return None }
        
        //read nibble from u64
        let src = src.to_le_bytes();
        let mut byte = src[(nibble_index/2) as usize];

        //pack into u8
        byte >>= (nibble_index & 1) * 4;
        byte |= nibble_index << 4;
        Some(Self(byte))
    }

    /// unpacks the crumb into a u64
    pub fn get_u64(&self) -> u64 {
        let crumb = self.0;
        let mut nibble = crumb << 4;
        let nibble_index = crumb >> 4;

        // shift to right side of byte if index is even
        nibble >>= ((nibble_index & 1) ^ 1) * 4;
        let mut bytes = [0u8; 8];
        bytes[(nibble_index/2) as usize] = nibble;
        u64::from_le_bytes(bytes)
    }

    /// returns inner u8, though I would recommend against using this right away to keep the the niceities of the type system
    pub fn get_u8(&self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod tests {

    use crate::Crumb;
    #[test]
    fn it_works() {
        let crumb = Crumb::new(1, 0b11111111).unwrap();
        assert_eq!(0b11110000u64, crumb.get_u64());
    }

    #[test]
    fn exaustive() {
        for index in 0..crate::MAX_NIBBLE_INDEX {
            for nibble in  1..16 {
                let shifted = nibble << index*4;

                let crumb = Crumb::new(index, shifted).unwrap();

                assert_eq!(shifted >> index*4, crumb.get_u64() >> index*4);
            }
        }
    }
}
