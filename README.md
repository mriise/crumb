[![License](https://img.shields.io/crates/l/crumb)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/crumb)](https://crates.io/crates/crumb)
[![Documentation](https://docs.rs/crumb/badge.svg?style=flat-square)](https://docs.rs/crumb)

Encode an index and value of a 4 bit [nibble](https://en.wikipedia.org/wiki/Nibble) from a u64 as a u8.

`|      4 bits     |     4 bits     |`

`| index of nibble | nibble of data |`

# Usage

```rust
use crumb::Crumb;

fn main() {
	let crumb = Crumb::new(1, 0b11111111).unwrap();
	assert_eq!(0b11110000u64, crumb.get_u64()); // external bits are discarded
}
```

### Name

A nibble of something is about the size of a crumb or something like that...

### Why?

Was doing some research on variable length integers and realized how nicely this fits.

### For What?

Not sure!

I don't think there are too many applications for this.
