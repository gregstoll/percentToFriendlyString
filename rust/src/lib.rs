use std::fmt;

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
    pub fn new(numerator: u8, denominator: u8) -> FriendlyProbability {
        FriendlyProbability {
            numerator: numerator,
            denominator: denominator,
            friendly_string: format!("{} in {}", numerator, denominator)
        }
    }

    pub fn from_probability(probability: f32) -> FriendlyProbability {
        // TODO
        FriendlyProbability::new(0, 1)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
