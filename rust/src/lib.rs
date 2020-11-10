use std::fmt;
#[macro_use]
extern crate lazy_static;

/// A probability that can be expressed in a user-friendly form.
/// The `friendly_string` property is a textual representation of
/// the probability (i.e. "5 out of 7")
#[derive(PartialEq, Eq, Debug)]
pub struct FriendlyProbability {
    numerator: u8,
    denominator: u8,
    friendly_description: &'static str,
    friendly_string: String
}

impl fmt::Display for FriendlyProbability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.friendly_string)
    }
}
impl FriendlyProbability {
    // http://xion.io/post/code/rust-optional-args.html
    /// Create a new FriendlyProbability.  Most of the time you will want to use `from_probability()` instead.
    pub fn new<T: Into<Option<String>>>(numerator: u8, denominator: u8, friendly_description: &'static str, friendly_string: T) -> FriendlyProbability {
        let real_friendly_string = friendly_string.into().unwrap_or_else(|| format!("{} in {}", numerator, denominator));
        FriendlyProbability {
            numerator,
            denominator,
            friendly_description,
            friendly_string: real_friendly_string
        }
    }
    /// Gets the numerator of the FriendlyProbability.
    pub fn numerator(self: &FriendlyProbability) -> u8 {
        self.numerator
    }
    /// Gets the denominator of the FriendlyProbability.
    pub fn denominator(self: &FriendlyProbability) -> u8 {
        self.denominator
    }
    /// Gets the friendly description of the FriendlyProbability.
    /// This is a qualitative description of the probability ("Still possible", "Flip a coin", "Good chance", etc.)
    pub fn friendly_description(self: &FriendlyProbability) -> &str {
        &self.friendly_description
    }
    /// Gets the friendly string of the FriendlyProbability.
    /// Usually this is the same as "{numerator} in {denominator}",
    /// but if the probability is very small it will instead be "<1 in 100",
    /// and if it's very large it will be ">99 in 100"
    pub fn friendly_string(self: &FriendlyProbability) -> &str {
        &self.friendly_string
    }
    /// Create a FriendlyProbability from an f32.
    /// # Examples
    /// 
    /// ```
    /// use probability_to_friendly_string::FriendlyProbability;
    /// 
    /// let friendly = FriendlyProbability::from_probability(0.723);
    /// assert_eq!(5, friendly.numerator());
    /// assert_eq!(7, friendly.denominator());
    /// assert_eq!("Good chance", friendly.friendly_description());
    /// assert_eq!("5 in 7", friendly.friendly_string());
    /// 
    /// let friendly = FriendlyProbability::from_probability(0.999);
    /// assert_eq!(">99 in 100", friendly.friendly_string());
    /// 
    /// let friendly = FriendlyProbability::from_probability(0.001);
    /// assert_eq!("<1 in 100", friendly.friendly_string());
    /// ```
    /// # Panics
    /// If probability is less than 0.0 or greater than 1.0.
    pub fn from_probability(probability: f32) -> FriendlyProbability {
        if probability < 0.0 || probability > 1.0 {
            panic!("probability is less than 0 or greater than 1!")
        }
        // use slice::binary_search_by
        // because f32's are not orderable
        // https://stackoverflow.com/questions/28247990/how-to-do-a-binary-search-on-a-vec-of-floats
        let friendly_description_location = FRIENDLY_DESCRIPTIONS.binary_search_by(|f| {
            f.0.partial_cmp(&probability).expect("Couldn't compare floats?")
        });
        let friendly_description = match friendly_description_location {
            Ok(i) => FRIENDLY_DESCRIPTIONS[i].1,
            Err(i) => FRIENDLY_DESCRIPTIONS[i - 1].1
        };
        if probability == 0.0 {
            return FriendlyProbability::new(0, 1, friendly_description, None)
        }
        if probability == 1.0 {
            return FriendlyProbability::new(1, 1, friendly_description, None)
        }
        if probability > 0.99 {
            return FriendlyProbability::new(99, 100, friendly_description, String::from(">99 in 100"))
        }
        if probability < 0.01 {
            return FriendlyProbability::new(1, 100, friendly_description, String::from("<1 in 100"))
        }
        let data = &FRACTION_DATA;
        let fraction_to_compare = Fraction::new_for_comparison(probability);
        // use slice::binary_search_by since we can't derive Ord
        // because f32's are not orderable
        // https://stackoverflow.com/questions/28247990/how-to-do-a-binary-search-on-a-vec-of-floats
        let location = data.binary_search_by(|f| {
            f.partial_cmp(&fraction_to_compare).expect("Couldn't compare values?")
        });
        fn friendly_probability_from_fraction(fraction: &Fraction, friendly_description: &'static str) -> FriendlyProbability {
            FriendlyProbability::new(fraction.numerator, fraction.denominator, friendly_description, None)
        }
        let data_len = data.len();
        match location {
            Ok(i) => friendly_probability_from_fraction(&data[i], friendly_description),
            Err(i) => {
                // This means it could be inserted at index i
                if i == 0 {
                    return friendly_probability_from_fraction(&data[0], friendly_description);
                }
                if i == data_len {
                    return friendly_probability_from_fraction(&data[data_len - 1], friendly_description);
                }
                if probability - (&data[i - 1]).value < (&data[i]).value - probability {
                    return friendly_probability_from_fraction(&data[i - 1], friendly_description);
                }
                else {
                    return friendly_probability_from_fraction(&data[i], friendly_description);
                }
            }
        }
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
const FRIENDLY_DESCRIPTIONS : [(f32, &str); 14] = [
    (0.0, "Hard to imagine"),
    (0.005, "Barely possible"),
    (0.02, "Still possible"),
    (0.08, "Some chance"),
    (0.15, "Could happen"),
    (0.2, "Perhaps"),
    (0.45, "Flip a coin"),
    (0.55, "Likelier than not"),
    (0.7, "Good chance"),
    (0.8, "Probably"),
    (0.85, "Quite likely"),
    (0.9, "Pretty likely"),
    (0.95, "Very likely"),
    (0.995, "Almost certainly"),
];

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
        for &d in [12, 15, 20, 30, 40, 50, 60, 80, 100].iter() {
            fractions.push(Fraction::new(1, d));
            fractions.push(Fraction::new(d - 1, d));
        }
        fractions.sort_unstable_by(|a,b| a.partial_cmp(b).unwrap());
        fractions
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn friendly_probability_string_matches_numeric_inputs() {
        let fp = FriendlyProbability::new(1, 2, "", None);
        assert_eq!("1 in 2", fp.friendly_string);
    }
    #[test]
    fn friendly_probability_string_matches_string_input() {
        let s = String::from("something weird");
        let fp = FriendlyProbability::new(1, 2, "", s.clone());
        assert_eq!(s, fp.friendly_string);
    }
    #[test]
    #[should_panic]
    fn friendly_probability_from_fraction_less_than_0() {
        FriendlyProbability::from_probability(-0.01);
    }
    #[test]
    #[should_panic]
    fn friendly_probability_from_fraction_greater_than_1() {
        FriendlyProbability::from_probability(1.01);
    }
}