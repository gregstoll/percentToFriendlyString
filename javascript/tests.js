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
        if (!expected.equals(actual)) {
            throw new Error("expected: " + expected.toString() + " actual: " + actual.toString() + " for \"" + line + "\" (line " + (i+1) + ")"); 
        }
    }
});
