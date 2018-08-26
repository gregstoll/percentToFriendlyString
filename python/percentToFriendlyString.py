class FriendlyPercentString:
	def __init__(self, numerator, denominator, friendlyString=None):
		self.numerator = numerator
		self.denominator = denominator
		self.friendlyString = "{0} in {1}".format(numerator, denominator) if friendlyString is None else friendlyString

	def __eq__(self, other):
		return self.numerator == other.numerator and self.denominator == other.denominator and self.friendlyString == other.friendlyString
	
	def __str__(self):
		return "{0}/{1} (text: \"{2}\")".format(self.numerator, self.denominator, self.friendlyString)
	def __repr__(self):
		return self.__str__()

def percentToFriendlyString(s):
	f = float(s)
	if (f == 0):
		return FriendlyPercentString(0, 1)
	if (f == 1):
		return FriendlyPercentString(1, 1)
	return FriendlyPercentString(0, 1)
