'use strict';

class FriendlyProbability {
    constructor(numerator, denominator, friendlyString) {
        this.numerator = numerator;
        this.denominator = denominator;
        this.friendlyString = friendlyString || (numerator + " in " + denominator);
    }

    toString() {
        return this.numerator + "/" + this.denominator + ' (text: "' + this.friendlyString + '")';
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
        if (f > .99) {
            return new FriendlyProbability(99, 100, ">99 in 100");
        }
        if (f < .01) {
            return new FriendlyProbability(1, 100, "<1 in 100");
        }
        /**
         *  * Return 0 <= i <= array.length such that array[i - 1] <= x && x <= array[i].
         *   */
        function binarySearch(array, x) {
            let lo = -1, hi = array.length;
            while (1 + lo < hi) {
                const mi = lo + ((hi - lo) >> 1);
                if (x <= array[mi][0]) {
                    hi = mi;
                } else {
                    lo = mi;
                }
           }
           return hi;
        }
        let data = FriendlyProbability._fractionsData;
        let right = binarySearch(data, f);
        let left = right - 1;
        if (left >= 0 && f - data[left][0] < data[right][0] - f) {
            return new FriendlyProbability(data[left][1], data[left][2]);
        }
        else {
            return new FriendlyProbability(data[right][1], data[right][2]);
        }
    }
}

function areRelativelyPrime(a, b) {
    function gcd(a, b) {
        let t = 0;
        while (b != 0) {
            t = a;
            a = b;
            b = t % b;
        }
        return a;
    }
    return gcd(a, b) == 1;
}

function initializeFractionsData() {
    let data = [];
    function addFraction(numerator, denominator) {
        data.push([numerator/denominator, numerator, denominator]);
    }
    for (let d = 2; d <= 10; ++d) {
        for (let n = 1; n < d; ++n) {
            if (areRelativelyPrime(n, d)) {
                addFraction(n, d);
            }
        }
    }
    for (let d of [12, 15, 20, 30, 40, 50, 60, 80, 100]) {
        addFraction(1, d);
        addFraction(d - 1, d);
    }
    data.sort();
    FriendlyProbability._fractionsData = data;
}
initializeFractionsData();

// Make this able to be consumed without node.js
if (typeof module !== 'undefined')
{
    module.exports = FriendlyProbability;
}
