Requires Python 3.5 or above

Usage:

    friendly = probabilityToFriendlyString.FriendlyProbability.fromProbability(.723);
    
`friendly` is a class of type `FriendlyProbability` that has properties
- `friendlyString`: a string representing the probability (in this case "5 in 7")
- `friendlyDescription`: a qualitative description of the probability ("Still possible", "Flip a coin", "Good chance", etc.)
- `numerator`: the numerator of the probability (in this case 5)
- `denominator`: the denominator of the probability (in this case 7)

Note that passing a value less than 0 or greater than 1 to `FriendlyProbability.fromProbability()` will raise an `ValueError`
