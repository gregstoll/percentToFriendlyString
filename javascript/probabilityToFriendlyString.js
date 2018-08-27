'use strict';

class FriendlyProbability {
    constructor(numerator, denominator, friendlyString) {
        this.numerator = numerator;
        this.denominator = denominator;
        this.friendlyString = friendlyString || (numerator + " in " + denominator);
    }

    toString() {
        return this.numerator + "/" + this.denominator + " (text: \"" + this.friendlyString + "\")";
    }
    equals(other) {
        return this.numerator == other.numerator && this.denominator == other.denominator && this.friendlyString == other.friendlyString;
    }

    static fromProbability(f) {
        if (f < 0 || f > 1) {
            throw new Error("out of range");
        }
        if (f == 0) {
            return new FriendlyProbability(0, 1);
        } 
        if (f == 1) {
            return new FriendlyProbability(1, 1);
        } 
        //TODO
        return new FriendlyProbability(1, 2);
    }
}

module.exports.FriendlyProbability = FriendlyProbability;
