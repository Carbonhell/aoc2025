# Advent of Code 2025

See [Advent of Code](https://adventofcode.com/2025/about).

My main goal with this is to have fun with this year's challenges and gain some more practice with:
- jj (used for VCS operations in this repository);
- functional programming (where it makes sense);

I may also use this project as a sandbox to try out some crates or services I've been eyeing to play with (again, in some cases) for a while:
- Mise
- [Ratatui](https://ratatui.rs/);

# Resources used

## Day 1
For the input reading procedure, I followed the example at [this link](https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html#a-more-efficient-approach) to reduce intermediate allocations.

For the wrapping operations around the [0:99] domain, Rust (and C) calculate the remainder in a different way than, say, Python for negative numbers. Rust calculates the remainder assigning it the same sign as the division (which is truncating). What we want instead is a nonnegative remainder, for which [`rem_euclid`](https://doc.rust-lang.org/std/primitive.i32.html#method.rem_euclid) does the job. See [here](https://users.rust-lang.org/t/why-works-differently-between-rust-and-python/83911/7) for more details.