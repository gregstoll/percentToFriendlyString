Package on NuGet: [ProbabilityToFriendlyString](https://www.nuget.org/packages/ProbabilityToFriendlyString/).  Requires .NET Standard 2.0. 

Usage:

    FriendlyProbability friendly = FriendlyProbability.FromProbability(.723);
    
`friendly` is a struct of type `FriendlyProbability` that has properties
- `FriendlyString`: a string representing the probability (in this case "5 in 7")
- `FriendlyDescription`: a string representing a qualitative description of the probability ("Still possible", "Flip a coin", "Good chance", etc.)
- `Numerator`: the numerator of the probability (in this case 5)
- `Denominator`: the denominator of the probability (in this case 7)

Note that passing a value less than 0 or greater than 1 to `FriendlyProbability.FromProbability()` will throw an `ArgumentOutOfRangeException`.
