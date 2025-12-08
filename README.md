# Advent of Code 2025

See [Advent of Code](https://adventofcode.com/2025/about).

My main goal with this is to have fun with this year's challenges and gain some more practice with:
- jj (used for VCS operations in this repository);
- functional programming (where it makes sense);

# Takehomes

## Day 1
For the input reading procedure, I followed the example at [this link](https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html#a-more-efficient-approach) to reduce intermediate allocations.

For the wrapping operations around the [0:99] domain, Rust (and C) calculate the remainder in a different way than, say, Python for negative numbers. Rust calculates the remainder assigning it the same sign as the division (which is truncating). What we want instead is a nonnegative remainder, for which [`rem_euclid`](https://doc.rust-lang.org/std/primitive.i32.html#method.rem_euclid) does the job. See [here](https://users.rust-lang.org/t/why-works-differently-between-rust-and-python/83911/7) for more details.

## Day 6

I was wondering why string parsing uses the FromStr trait instead of TryFrom. The post on URLO [here](https://users.rust-lang.org/t/is-there-actually-a-semantic-difference-between-fromstr-and-tryfrom-str/92765/16) and [here](https://users.rust-lang.org/t/is-it-bad-practice-to-implement-from-str-instead-of-fromstr/76753/4) explains the difference being based on semantics:
- TryFrom<T> describes that T can be *converted* to Self. If T is &str, it would mean that any string slice can be converted to Self. This makes sense for different views of string slices, such as ByteStr, OsStr, and it implies that Self is itself a representation of a string.
- FromStr describes that Self can be *parsed* from T. It does *not* imply that Self is a representation of a string, but it allows constructing an instance of Self from a string.

Related libs issue: [here](https://github.com/rust-lang/libs-team/issues/296) 