use probability_to_friendly_string::*;

#[test]
fn all_cases() {
    use std::fs;
    let contents = fs::read_to_string("../testCases.txt").expect("Failed to open testCases.txt");
    let lines = contents.lines();
    let mut test_cases = 0;
    for line in lines {
        if line.starts_with("#") || line.trim().len() == 0 {
            continue;
        }
        let parts : Vec<&str> = line.trim().split(",").collect();
        let numerator = parts[1].parse::<u8>().expect("parsing numerator");
        let denominator = parts[2].parse::<u8>().expect("parsing denominator");
        let expected : FriendlyProbability;
        if parts.len() == 3 {
            expected = FriendlyProbability::new(numerator, denominator, "", None);
        } else if parts.len() == 4 {
            expected = FriendlyProbability::new(numerator, denominator, "", parts[3].to_string());
        } else {
            panic!(format!("Got string with wrong number of parts: {}", line.trim()));
        }
        let probability = parts[0].parse::<f32>().expect("parsing probability");
        let friendly_probability = FriendlyProbability::from_probability(probability);
        assert_eq!(expected.numerator(), friendly_probability.numerator());
        assert_eq!(expected.denominator(), friendly_probability.denominator());
        assert_eq!(expected.friendly_string(), friendly_probability.friendly_string());
        test_cases += 1;
    }
    println!("Ran {} test cases", test_cases);
    assert!(test_cases > 0);
}

#[test]
fn all_cases_friendly_description() {
    use std::fs;
    let contents = fs::read_to_string("../testCases.friendlyDescription.txt").expect("Failed to open testCases.friendlyDescription.txt");
    let lines = contents.lines();
    let mut test_cases = 0;
    for line in lines {
        if line.starts_with("#") || line.trim().len() == 0 {
            continue;
        }
        let parts : Vec<&str> = line.trim().split(",").collect();
        let expected_friendly_description = parts[1];
        let probability = parts[0].parse::<f32>().expect("parsing probability");
        let friendly_probability = FriendlyProbability::from_probability(probability);
        assert_eq!(expected_friendly_description, friendly_probability.friendly_description());
        test_cases += 1;
    }
    println!("Ran {} test cases", test_cases);
    assert!(test_cases > 0);
}