# probabilityToFriendlyString

Make probabilities more intuitive by converting them to odds. (i.e. 72.3% becomes "5 in 7")

[Live demo here](https://gregstoll.github.io/probabilityToFriendlyString/)

**Python 3**: done (requires Python 3.5 or above)

**Javascript**: done, on npm: [probability-to-friendly-string](https://www.npmjs.com/package/probability-to-friendly-string) (ES6, tests use node.js)

**C#**: done, on NuGet: [ProbabilityToFriendlyString](https://www.nuget.org/packages/ProbabilityToFriendlyString/) (.NET Standard 2.0)

This project was inspired by [FiveThirtyEight's 2018 house forecast](https://projects.fivethirtyeight.com/2018-midterm-election-forecast/house/), where they use this friendlier way of showing probabilities.  I decided to reverse-engineer the algorithm they used and make it easily available to incorporate in other visualizations, etc.
