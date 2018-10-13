Gem [probability_to_friendly_string](https://rubygems.org/gems/probability_to_friendly_string), requires Ruby 2.3.1 or above.

Usage:

    require 'probability_to_friendly_string'
    friendly = ProbabilityToFriendlyString::FriendlyProbability.fromProbability .723
    
`friendly` is a class of type `FriendlyProbability` that has properties
- `friendlyString`: a string representing the probability (in this case "5 in 7")
- `numerator`: the numerator of the probability (in this case 5)
- `denominator`: the denominator of the probability (in this case 7)

Note that passing a value less than 0 or greater than 1 to `FriendlyProbability.fromProbability()` will raise an `RangeError`
