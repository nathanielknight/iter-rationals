# iter-rationals

[![crate](https://img.shields.io/crates/v/iter-rationals.svg)](https://crates.io/crates/iter-rationals)
[![documentation](https://docs.rs/iter-rationals/badge.svg)](https://docs.rs/iter-rationals)
[![main](https://github.com/nathanielknight/iter-rationals/actions/workflows/main.yml/badge.svg)](https://github.com/nathanielknight/iter-rationals/actions/workflows/main.yml)

This crate implements an iterator over the [rational numbers] which:

* yields each number once-and-only-once,
* requires only a fixed amount of memory, and
* yields successive values with a fixed amount of arithmetic operations.

[rational numbers]: https://simple.wikipedia.org/wiki/Rational_number

The algorithm is described in [Functional Pearl: Enumerating the Rationals] by
Gibbons, Lester, and Bird; see the paper for an explanation of how it works.

[Functional Pearl: Enumerating the Rationals]: http://www.cs.ox.ac.uk/people/jeremy.gibbons/publications/rationals.pdf

# Usage

```rust
use iter_rationals::Rationals;

fn main() {
    let rs = Rationals::<u32>::new();

    for r in rs.take(20) {
        println!("{r}");
    }
}
```

# Installation

Add this dependency to your `Cargo.toml`:

```
iter-rationals = "0.1"
```

or on the command line:

```
cargo add iter-rationals
```


# License
