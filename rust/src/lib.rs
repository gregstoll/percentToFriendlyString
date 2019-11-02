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
    use super::*;

    #[test]
    fn all_cases() {
        use std::fs;
        let contents = fs::read_to_string("../testCases.txt").expect("Failed to open testCases.txt");
        let lines = contents.lines();
        let mut test_cases = 0;
        for line in lines {
            if line.starts_with("#") {
                continue;
            }
            let parts : Vec<&str> = line.trim().split(",").collect();
            let numerator = parts[1].parse::<u8>().expect("parsing numerator");
            let denominator = parts[2].parse::<u8>().expect("parsing denominator");
            let expected : FriendlyProbability;
            if parts.len() == 3 {
                expected = FriendlyProbability::new(numerator, denominator);
            } else if parts.len() == 4 {
                expected = FriendlyProbability {
                    numerator: numerator,
                    denominator: denominator,
                    friendly_string: parts[3].to_string()
                 };
            } else {
                panic!(format!("Got string with wrong number of parts: {}", line.trim()));
            }
            let probability = parts[0].parse::<f32>().expect("parsing probability");
            assert_eq!(FriendlyProbability::from_probability(probability), expected);
            test_cases += 1;
        }
        println!("Ran {} test cases", test_cases);
        assert!(test_cases > 0);
    }
}
