//! Encode an index of a [nibble](https://en.wikipedia.org/wiki/Nibble) from a u64 as a u8.
//! 
//! `|      4 bits     |     4 bits     |`
//! 
//! `| index of nibble | nibble of data |`
//! 
//! # Usage
//! 
//! ```rust
//! use crate::CrumbExt;
//! 
//! fn main() {
//!     let test: u64 = 0b11111111;
//!     // let crumb: u8 = unsafe { test.get_unchecked_crumb(1) };
//!     let crumb: u8 = test.get_crumb(1).unwrap();
//!     assert_eq!(0b11110000u64, u64::from_crumb(crumb));
//! }
//! ```
//! 
//! ### Name
//! 
//! A nibble of something is about the size of a crumb or something like that...
//! 
//! ### How?
//! 
//! Was doing some research on variable length integers and realized how nicely this fits.
//! 
//! ### Why?
//! 
//! I don't think there are too many applications for this, I have one in mind but that is an entire project in itself.

const MAX_NIBBLE_INDEX: u8 = 1u8 << 4;
/// this is NOT intended to be implemented in anything but u64, it is only pub for documentation
pub trait CrumbExt {
    unsafe fn get_unchecked_crumb(&self, nibble_index: u8) -> u8;
    /// None where `nibble_index >= 1u8 << 4`
    fn get_crumb(&self, nibble_index: u8) -> core::option::Option<u8>;
    fn from_crumb(crumb: u8) -> Self;
}

impl CrumbExt for u64 {
    unsafe fn get_unchecked_crumb(&self, nibble_index: u8) -> u8 {
        //read nibble from u64
        let src = self.to_le_bytes();
        let mut byte = src[(nibble_index/2) as usize];
        //pack into u8
        byte >>= (nibble_index & 1) * 4;
        byte |= nibble_index << 4;
        byte
    }
    fn get_crumb(&self, nibble_index: u8) -> core::option::Option<u8> {
        if nibble_index >= MAX_NIBBLE_INDEX {return None}
        unsafe { Some(self.get_unchecked_crumb(nibble_index)) }
    }
    fn from_crumb(crumb: u8) -> Self {
        let mut nibble = crumb << 4;
        let nibble_index = crumb >> 4;
        // shift to right side of byte if index is even
        nibble >>= ((nibble_index & 1) ^ 1) * 4;
        let mut bytes = [0u8; 8];
        bytes[(nibble_index/2) as usize] = nibble;
        Self::from_le_bytes(bytes)
    }
}


#[cfg(test)]
mod tests {
    use crate::CrumbExt;
    #[test]
    fn it_works() {
        let test: u64 = 0b11111111;
        let crumb: u8 = test.get_crumb(1).unwrap();
        assert_eq!(0b11110000u64, u64::from_crumb(crumb));
    }

    #[test]
    fn exaustive() {
        for x in 0..(1u8 << 4) {
            for i in  1..16 {
                let shifted = i << x*4;
                println!("\ntesting nibble_index {} for {}", x, shifted);
                let crumb = shifted.get_crumb(x).unwrap();

                assert_eq!(shifted >> x*4, u64::from_crumb(crumb) >> x*4);
            }
        }
    }
}
