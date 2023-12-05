# day 5

I did not bother to optimize this one. Run it with `cargo run --release` should take less than 5 minutes.

# Possible Optimization

- Prune the input ranges, they seem to have overlap.
- Processing the ranges as a whole instead of seed by seed. Apply a mapping to a range would mean
  it would be split in multiple ranges.

```rust

impl Mapping {
    pub fn process(&self, range: (usize,usize)) -> Vec<(usize,usize)> {
        .....
    }
}

```
