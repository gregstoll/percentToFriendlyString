using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using Xunit;

namespace Gregstoll.ProbabilityToFriendlyString
{
    [Serializable]
    public class FriendlyProbabilityTests
    {
        public struct TestCase
        {
            public int LineNumber { get; }
            public double Input { get; }
            public FriendlyProbability Output { get; }
            public TestCase(int lineNumber, double input, FriendlyProbability output)
            {
                LineNumber = lineNumber;
                Input = input;
                Output = output;
            }
        }

        public struct TestCaseFriendlyDescription
        {
            public int LineNumber { get; }
            public double Input { get; }
            public string ExpectedFriendlyDescription { get; }
            public TestCaseFriendlyDescription(int lineNumber, double input, string expectedFriendlyDescription)
            {
                LineNumber = lineNumber;
                Input = input;
                ExpectedFriendlyDescription = expectedFriendlyDescription;
            }
        }

        [Theory]
        [MemberData(nameof(GetTestCases))]
        public void TestsFromFile(TestCase testCase)
        {
            var actual = FriendlyProbability.FromProbability(testCase.Input);
            if (testCase.Output != actual)
            {
                Assert.True(false, string.Format("expected {0} actual {1} from line {2}", testCase.Output, actual, testCase.LineNumber));
            }
        }

        [Theory]
        [MemberData(nameof(GetFriendlyDescriptionTestCases))]
        public void FriendlyDescriptionTestsFromFile(TestCaseFriendlyDescription testCase)
        {
            var actual = FriendlyProbability.FromProbability(testCase.Input);
            if (testCase.ExpectedFriendlyDescription != actual.FriendlyDescription)
            {
                Assert.True(false, string.Format("expected {0} actual {1} from line {2}", testCase.ExpectedFriendlyDescription, actual.FriendlyDescription, testCase.LineNumber));
            }
        }

       public static IEnumerable<object[]> GetTestCases()
       {
            List<TestCase> testCases = new List<TestCase>();
            string path = Path.GetDirectoryName(typeof(FriendlyProbabilityTests).Assembly.Location);
            string testCasesPath = null;
            while (testCasesPath == null)
            {
                string tempPath = Path.Combine(path, "testCases.txt");
                if (File.Exists(tempPath))
                {
                    testCasesPath = tempPath;
                }
                path = Path.GetDirectoryName(path);
            }
            var lines = File.ReadAllLines(testCasesPath);
            int lineNumber = 0;
            foreach (string rawLine in lines)
            {
                ++lineNumber;
                string line = rawLine.Trim();
                if (line.StartsWith("#"))
                {
                    continue;
                }
                var parts = line.Split(',');
                double input = double.Parse(parts[0]);
                if (parts.Length == 3)
                {
                    testCases.Add(new TestCase(lineNumber, input, new FriendlyProbability(byte.Parse(parts[1]), byte.Parse(parts[2]), string.Empty)));
                }
                else if (parts.Length == 4)
                {
                    testCases.Add(new TestCase(lineNumber, input, new FriendlyProbability(byte.Parse(parts[1]), byte.Parse(parts[2]), string.Empty, parts[3])));
                }
                else
                {
                    throw new InvalidDataException(string.Format("line improperly formatted (line {0}: {1}", lineNumber, line));
                }
            }

            return testCases.Select(tc => new object[] { tc });
        }

       public static IEnumerable<object[]> GetFriendlyDescriptionTestCases()
       {
            List<TestCaseFriendlyDescription> testCases = new List<TestCaseFriendlyDescription>();
            string path = Path.GetDirectoryName(typeof(FriendlyProbabilityTests).Assembly.Location);
            string testCasesPath = null;
            while (testCasesPath == null)
            {
                string tempPath = Path.Combine(path, "testCases.friendlyDescription.txt");
                if (File.Exists(tempPath))
                {
                    testCasesPath = tempPath;
                }
                path = Path.GetDirectoryName(path);
            }
            var lines = File.ReadAllLines(testCasesPath);
            int lineNumber = 0;
            foreach (string rawLine in lines)
            {
                ++lineNumber;
                string line = rawLine.Trim();
                if (line.StartsWith("#"))
                {
                    continue;
                }
                var parts = line.Split(',');
                double input = double.Parse(parts[0]);
                testCases.Add(new TestCaseFriendlyDescription(lineNumber, input, parts[1]));
            }

            return testCases.Select(tc => new object[] { tc });
        }
    }
}
