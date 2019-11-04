use std::fmt;
#[macro_use]
extern crate lazy_static;

#[derive(PartialEq, Eq, Debug)]
pub struct FriendlyProbability {
    numerator: u8,
    denominator: u8,
    friendly_string: String
}

impl fmt::Display for FriendlyProbability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.friendly_string)
    }
}
impl FriendlyProbability {
    // http://xion.io/post/code/rust-optional-args.html
    pub fn new<T: Into<Option<String>>>(numerator: u8, denominator: u8, friendly_string: T) -> FriendlyProbability {
        let real_friendly_string = friendly_string.into().unwrap_or_else(|| format!("{} in {}", numerator, denominator));
        FriendlyProbability {
            numerator,
            denominator,
            friendly_string: real_friendly_string
        }
    }
    pub fn numerator(self: &FriendlyProbability) -> u8 {
        self.numerator
    }
    pub fn denominator(self: &FriendlyProbability) -> u8 {
        self.denominator
    }
    pub fn friendly_string(self: &FriendlyProbability) -> &str {
        &self.friendly_string
    }

    pub fn from_probability(probability: f32) -> FriendlyProbability {
        // TODO
        FriendlyProbability::new(0, 1, None)
    }
}

// TODO - use slice::binary_search_by on these since we can't derive Ord
// because f32's are not orderable
// https://stackoverflow.com/questions/28247990/how-to-do-a-binary-search-on-a-vec-of-floats
#[derive(PartialOrd, PartialEq)]
struct Fraction {
    value: f32,
    numerator: u8,
    denominator: u8,
}
impl Fraction {
    pub fn new(numerator: u8, denominator: u8) -> Fraction {
        Fraction {
            numerator: numerator,
            denominator: denominator,
            value: numerator as f32/denominator as f32
        }
    }
}

lazy_static! {
    static ref FRACTION_DATA: Vec<Fraction> = {
        let mut d : Vec<Fraction> = Vec::new();
        d.push(Fraction::new(1, 2));
        // TODO - more fractions
        // TODO - make sure this is in ascending order
        d.sort_unstable_by(|a,b| a.partial_cmp(b).unwrap());
        d
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn friendly_probability_string_matches_numeric_inputs() {
        let fp = FriendlyProbability::new(1, 2, None);
        assert_eq!("1 in 2", fp.friendly_string);
    }
    #[test]
    fn friendly_probability_string_matches_string_input() {
        let s = String::from("something weird");
        let fp = FriendlyProbability::new(1, 2, s.clone());
        assert_eq!(s, fp.friendly_string);
    }
}
