Package on npm: [probability-to-friendly-string](https://www.npmjs.com/package/probability-to-friendly-string).  Requires ES6, can be used with or without node.js.

Usage:
```javascript
// This line isn't needed if you're not using through npm
import FriendlyProbability from 'probability-to-friendly-string';
let friendly = FriendlyProbability.fromProbability(.723);
```    

`friendly` is a class of type `FriendlyProbability` that has properties
- `friendlyString`: a string representing the probability (in this case "5 in 7")
- `friendlyDescription`: a qualitative description of the probability ("Still possible", "Flip a coin", "Good chance", etc.)
- `numerator`: the numerator of the probability (in this case 5)
- `denominator`: the denominator of the probability (in this case 7)

Note that passing a value less than 0 or greater than 1 to `FriendlyProbability.fromProbability()` will throw an Error.
