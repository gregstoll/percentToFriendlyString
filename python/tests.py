#!/usr/bin/python3
import unittest, probabilityToFriendlyString

class TestPercentToFriendlyString(unittest.TestCase):
	def test_allCases(self):
		with open('../testCases.txt', 'r') as f:
			lineNumber = 0
			for line in f.readlines():
				lineNumber += 1
				self.lineTest(line, lineNumber)

	def lineTest(self, line, lineNumber):
		line = line.strip()
		if line.startswith('#'):
			return
		parts = line.split(',')
		if len(parts) == 3:
			expected = probabilityToFriendlyString.FriendlyProbability(int(parts[1]), int(parts[2]))
		elif len(parts) == 4:
			expected = probabilityToFriendlyString.FriendlyProbability(int(parts[1]), int(parts[2]), parts[3])
		else:
			self.fail("Line badly formatted: {0} (line {1})".format(line.strip(), lineNumber))
		actual = probabilityToFriendlyString.FriendlyProbability.fromProbability(float(parts[0]))
		self.assertEqual(expected, actual, "Called on {0} (line {1})".format(parts[0], lineNumber))


if __name__ == '__main__':
	unittest.main()
