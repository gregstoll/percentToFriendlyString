package com.gregstoll.probabilityToFriendlyString;

import static org.junit.Assert.assertEquals;
import org.junit.Test;
import java.io.BufferedReader;
import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.IOException;
import java.net.URL;
import java.util.ArrayList;

public class Tests {
    @Test
    public void testCases() throws FileNotFoundException, IOException {
        ClassLoader classLoader = Tests.class.getClassLoader();
        URL classLocation = classLoader.getResource("com/gregstoll/probabilityToFriendlyString/Tests.class");
        File testCasesDirectory = new File(classLocation.getPath()).getParentFile();
        while (!(new File(testCasesDirectory, "testCases.txt").exists())) {
            testCasesDirectory = testCasesDirectory.getParentFile();
        }
        File testCasesFile = new File(testCasesDirectory, "testCases.txt");
        FileReader testCasesFileReader = new FileReader(testCasesFile);
        BufferedReader bufferedReader = new BufferedReader(testCasesFileReader);
        
        String line;
        int lineNumber = 1;
        ArrayList<TestCase> testCases = new ArrayList<TestCase>();
        while((line = bufferedReader.readLine()) != null) {
            if (line.startsWith("#")) {
                lineNumber++;
                continue;
            }
            String[] parts = line.split(",");
            double input = Double.parseDouble(parts[0]);
            if (parts.length == 3) {
                testCases.add(new TestCase(lineNumber, input, new FriendlyProbability(Byte.parseByte(parts[1]), Byte.parseByte(parts[2]), "")));
            }
            else if (parts.length == 4) {
                testCases.add(new TestCase(lineNumber, input, new FriendlyProbability(Byte.parseByte(parts[1]), Byte.parseByte(parts[2]), "", parts[3])));
            }
            else {
                throw new IOException(String.format("line %d improperly formatted: %s", lineNumber, line));
            }
            lineNumber++;
        }   
        bufferedReader.close();
        for (TestCase testCase : testCases) {
            FriendlyProbability actual = FriendlyProbability.fromProbability(testCase.getInput());
            assertEquals(String.format("numerator on line %d", testCase.getLineNumber()), testCase.getOutput().getNumerator(), actual.getNumerator());
            assertEquals(String.format("denominator on line %d", testCase.getLineNumber()), testCase.getOutput().getDenominator(), actual.getDenominator());
            assertEquals(String.format("friendlyString on line %d", testCase.getLineNumber()), testCase.getOutput().getFriendlyString(), actual.getFriendlyString());
        }
    }

    @Test
    public void testCasesForFriendlyDescription() throws FileNotFoundException, IOException {
        ClassLoader classLoader = Tests.class.getClassLoader();
        URL classLocation = classLoader.getResource("com/gregstoll/probabilityToFriendlyString/Tests.class");
        File testCasesDirectory = new File(classLocation.getPath()).getParentFile();
        while (!(new File(testCasesDirectory, "testCases.friendlyDescription.txt").exists())) {
            testCasesDirectory = testCasesDirectory.getParentFile();
        }
        File testCasesFile = new File(testCasesDirectory, "testCases.friendlyDescription.txt");
        FileReader testCasesFileReader = new FileReader(testCasesFile);
        BufferedReader bufferedReader = new BufferedReader(testCasesFileReader);

        String line;
        int lineNumber = 1;
        ArrayList<TestCaseFriendlyDescription> testCases = new ArrayList<TestCaseFriendlyDescription>();
        while((line = bufferedReader.readLine()) != null) {
            if (line.startsWith("#")) {
                lineNumber++;
                continue;
            }
            String[] parts = line.split(",");
            double input = Double.parseDouble(parts[0]);
            testCases.add(new TestCaseFriendlyDescription(lineNumber, input, parts[1]));
            lineNumber++;
        }
        bufferedReader.close();
        for (TestCaseFriendlyDescription testCase : testCases) {
            FriendlyProbability actual = FriendlyProbability.fromProbability(testCase.getInput());
            assertEquals(String.format("line %d", testCase.getLineNumber()), testCase.getExpectedDescription(), actual.getFriendlyDescription());
        }
    }

    private class TestCase {
        private int _lineNumber;
        private double _input;
        private FriendlyProbability _output;
        public TestCase(int lineNumber, double input, FriendlyProbability output) {
            _lineNumber = lineNumber;
            _input = input;
            _output = output;
        }
        public int getLineNumber() {
            return _lineNumber;
        }
        public double getInput() {
            return _input;
        }
        public FriendlyProbability getOutput() {
            return _output;
        }
    }

    private class TestCaseFriendlyDescription {
        private int _lineNumber;
        private double _input;
        private String _expectedDescription;
        public TestCaseFriendlyDescription(int lineNumber, double input, String expectedDescription) {
            _lineNumber = lineNumber;
            _input = input;
            _expectedDescription = expectedDescription;
        }
        public int getLineNumber() {
            return _lineNumber;
        }
        public double getInput() {
            return _input;
        }
        public String getExpectedDescription() {
            return _expectedDescription;
        }
    }
}
