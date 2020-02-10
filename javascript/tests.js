'use strict';

var fs = require("fs");
var FriendlyProbability = require("./probabilityToFriendlyString");

fs.readFile("../testCases.txt", "utf8", function(error, data) {
    let lines = data.split("\n");
    for (let i = 0; i < lines.length; ++i) {
        let line = lines[i].trim();
        if (line.startsWith("#") || line.length == 0) {
            continue;
        }
        let parts = line.split(",");
        let expected = new FriendlyProbability(parseInt(parts[1]), parseInt(parts[2]), parts[3]);
        let actual = FriendlyProbability.fromProbability(parseFloat(parts[0]));
        if (!(expected.numerator == actual.numerator
              || expected.denominator == actual.denominator
              || expected.friendlyString == actual.friendlyString)) {
            throw new Error("probability: expected: " + expected.toString() + " actual: " + actual.toString() + " for \"" + line + "\" (line " + (i+1) + ")"); 
        }
    }
});

fs.readFile("../testCases.friendlyDescription.txt", "utf8", function(error, data) {
    let lines = data.split("\n");
    for (let i = 0; i < lines.length; ++i) {
        let line = lines[i].trim();
        if (line.startsWith("#") || line.length == 0) {
            continue;
        }
        let parts = line.split(",");
        let expected = parts[1];
        let actual = FriendlyProbability.fromProbability(parseFloat(parts[0]));
        if (expected.friendlyDescription !== actual.friendlyDescription) {
            throw new Error("friendlyDescription: expected: " + expected.friendlyDescription + " actual: " + actual.friendlyDescription + " for \"" + line + "\" (line " + (i+1) + ")"); 
        }
    }
});
