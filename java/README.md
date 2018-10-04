Built against JDK 8

Usage:

    FriendlyProbability friendly = FriendlyProbability.fromProbability(.723);    

`friendly` is a class of type `FriendlyProbability` with getters
- `getFriendlyString()`: a string representing the probability (in this case "5 in 7")
- `getNumerator()`: the numerator of the probability (in this case 5)
- `getDenominator()`: the denominator of the probability (in this case 7)

Note that passing a value less than 0 or greater than 1 to `FriendlyProbability.fromProbability()` will throw an `IllegalArgumentException`.
