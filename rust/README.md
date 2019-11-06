Published as [probability_to_friendly_string](https://crates.io/crates/probability_to_friendly_string) crate.

Requires Ruby 2018 edition

Usage:
```rust
use probability_to_friendly_string::FriendlyProbability;

let friendly = FriendlyProbability::from_probability(0.723);
assert_eq!(5, friendly.numerator());
assert_eq!(7, friendly.denominator());
assert_eq!("5 in 7", friendly.friendly_string());

let friendly = FriendlyProbability::from_probability(0.999);
assert_eq!(">99 in 100", friendly.friendly_string());

let friendly = FriendlyProbability::from_probability(0.001);
assert_eq!("<1 in 100", friendly.friendly_string());
```

Note that passing a value less than 0.0 or greater than 1.0 to `FriendlyProbability::from_probability` will panic.
