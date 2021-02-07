[![License](https://img.shields.io/crates/l/crumb)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/crumb)](https://crates.io/crates/crumb)
[![Documentation](https://docs.rs/multibase/badge.svg?style=flat-square)](https://docs.rs/multibase)

Encode an index of a [nibble](https://en.wikipedia.org/wiki/Nibble) from a u64 as a u8.

`|      4 bits     |     4 bits     |`

`| index of nibble | nibble of data |`

# Usage

```rust
use crate::CrumbExt;

fn main() {
	let test: u64 = 0b11111111;
	// let crumb: u8 = unsafe { test.get_unchecked_crumb(1) };
	let crumb: u8 = test.get_crumb(1).unwrap();
	assert_eq!(0b11110000u64, u64::from_crumb(crumb));
}
```

### Name

A nibble of something is about the size of a crumb or something like that...

### How?

Was doing some research on variable length integers and realized how nicely this fits.

### Why?

I don't think there are too many applications for this, I have one in mind but that is an entire project in itself.