Requires PHP 7.0 or above.  Running tests requires [PHPUnit 6.5](https://phpunit.de/getting-started/phpunit-6.html).

Usage:

    $friendly = FriendlyProbability::fromProbability(.723);
    
`$friendly` is an object of type `FriendlyProbabilty` that has properties
- `friendly_string`: a string representing the probability (in this case "5 in 7")
- `friendly_description`: a string representing a qualitative description of the probability (in this case "Good chance")
- `numerator`: the numerator of the probability (in this case 5)
- `denominator`: the denominator of the probability (in this case 7)

Note that passing a value less than 0 or greater than 1 to `FriendlyProbability::fromProbability()` will raise an `DomainException`
