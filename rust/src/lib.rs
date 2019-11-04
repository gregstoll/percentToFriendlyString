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
        if probability < 0.0 || probability > 1.0 {
            panic!("probability is less than 0 or greater than 1!")
        }
        if probability == 0.0 {
            return FriendlyProbability::new(0, 1, None)
        }
        if probability == 1.0 {
            return FriendlyProbability::new(1, 1, None)
        }
        if probability > 0.99 {
            return FriendlyProbability::new(99, 100, String::from(">99 in 100"))
        }
        if probability < 0.01 {
            return FriendlyProbability::new(1, 100, String::from("<1 in 100"))
        }
        let data = &FRACTION_DATA;
        let fraction_to_compare = Fraction::new_for_comparison(probability);
        // use slice::binary_search_by since we can't derive Ord
        // because f32's are not orderable
        // https://stackoverflow.com/questions/28247990/how-to-do-a-binary-search-on-a-vec-of-floats
        let location = data.binary_search_by(|f| {
            f.partial_cmp(&fraction_to_compare).expect("Couldn't compare values?")
        });
        // TODO
        //match location {
         //   OK(i) => 
        //}
        FriendlyProbability::new(0, 1, None)
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
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
    fn new_for_comparison(value: f32) -> Fraction {
        Fraction {
            numerator: 0,
            denominator: 0,
            value
        }
    }
}

lazy_static! {
    static ref FRACTION_DATA: Vec<Fraction> = {
        let mut fractions : Vec<Fraction> = Vec::new();
        fn gcd(x: u8, y: u8) -> u8 {
            let mut x = x;
            let mut y = y;
            while y != 0 {
                let t = y;
                y = x % y;
                x = t;
            }
            x
        }
        for d in 2..11 {
            for n in 1..d {
                if gcd(n, d) == 1 {
                    fractions.push(Fraction::new(n, d));
                }
            }
        }
        // TODO - more fractions
        fractions.sort_unstable_by(|a,b| a.partial_cmp(b).unwrap());
        fractions
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
