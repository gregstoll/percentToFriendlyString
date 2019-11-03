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
    pub fn new_with_default_string(numerator: u8, denominator: u8) -> FriendlyProbability {
        FriendlyProbability::new(numerator, denominator, format!("{} in {}", numerator, denominator))
    }
    pub fn new(numerator: u8, denominator: u8, friendly_string: String) -> FriendlyProbability {
        FriendlyProbability {
            numerator,
            denominator,
            friendly_string
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
        FriendlyProbability::new_with_default_string(0, 1)
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
    fn makeFriendlyProbability_FriendlyStringMatchesInputs() {
        let fp = FriendlyProbability::new_with_default_string(1, 2);
        assert_eq!("1 in 2", fp.friendly_string);
    }
}
