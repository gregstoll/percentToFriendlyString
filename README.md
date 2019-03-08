# probabilityToFriendlyString

Make probabilities more intuitive by converting them to odds. (i.e. 72.3% becomes "5 in 7")

[Live demo here](https://gregstoll.github.io/probabilityToFriendlyString/)

**Javascript**: on npm: [probability-to-friendly-string](https://www.npmjs.com/package/probability-to-friendly-string) (ES6, tests use node.js)

**Python 3**: requires Python 3.5 or above

**C#**: on NuGet: [ProbabilityToFriendlyString](https://www.nuget.org/packages/ProbabilityToFriendlyString/) (.NET Standard 2.0)

**Java**: built with JDK 8

**Ruby**: gem [probability_to_friendly_string](https://rubygems.org/gems/probability_to_friendly_string), requires Ruby 2.3.1 or above

**PHP**: requires PHP 7.0 or above (tests require PHPUnit 6.5)

**LabVIEW NXG**: requires [LabVIEW NXG](https://ni.com/labview) 3.0 or later

This project was inspired by [FiveThirtyEight's 2018 house forecast](https://projects.fivethirtyeight.com/2018-midterm-election-forecast/house/), where they use this friendlier way of showing probabilities.  I decided to reverse-engineer the algorithm they used and make it easily available to incorporate in other visualizations, etc.
