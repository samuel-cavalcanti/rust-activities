# how to print Vec\<u8> like python

## Question

How to print Vec\<u8> like python ?

```rust

let bytes = b"hello";

println!("{} word", byte_format!(bytes) );
// expected output: b"hello" world;
```

## Answer

```rust
use std::fmt;

struct BytesFmt<T>(T);

impl<T: AsRef<[u8]>> fmt::Display for BytesFmt<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("b\"")?;
        for &e in self.0.as_ref() {
            std::ascii::escape_default(e).fmt(f)?;
        }
        f.write_str("\"")
    }
}

fn main() {
    let hello = b"hello";
    println!("{}, world!", BytesFmt(hello));
}
```
