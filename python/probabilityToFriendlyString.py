import bisect, math

class CachedStaticProperty:
    """Works like @property and @staticmethod combined"""

    def __init__(self, func):
        self.func = func

    def __get__(self, inst, owner):
        result = self.func()
        setattr(owner, self.func.__name__, result)
        return result

class FriendlyProbability:
	def __init__(self, numerator, denominator, friendlyDescription, friendlyString=None):
		self.numerator = numerator
		self.denominator = denominator
		self.friendlyDescription = friendlyDescription
		self.friendlyString = "{0} in {1}".format(numerator, denominator) if friendlyString is None else friendlyString

	def __eq__(self, other):
		return self.numerator == other.numerator \
		 and self.denominator == other.denominator \
		 and self.friendlyDescription == other.friendlyDescription \
		 and self.friendlyString == other.friendlyString
	
	def __str__(self):
		return "{0}/{1} (text: \"{2}\", description=\"{3}\")".format(self.numerator, self.denominator, self.friendlyString, self.friendlyDescription)
	def __repr__(self):
		return self.__str__()

	_friendlyDescriptionValues = [0.005, 0.02, 0.08, 0.15, 0.2, 0.45, 0.55, 0.7, 0.8, 0.85, 0.9, 0.95, 0.995]

	_friendlyDescriptionStrings = [
		"Hard to imagine", \
		'Barely possible', \
		'Still possible', \
		'Some chance', \
		'Could happen', \
		'Perhaps', \
		'Flip a coin', \
		'Likelier than not', \
		'Good chance', \
		'Probably', \
		'Quite likely', \
		'Pretty likely', \
		'Very likely', \
		'Almost certainly', \
		]

	@CachedStaticProperty
	def _fractionsData():
		data = []
		def addFraction(numerator, denominator):
			data.append((float(numerator)/denominator, numerator, denominator))
		for d in range(2, 11):
			for n in range(1, d):
				if (math.gcd(n, d) == 1):
					addFraction(n, d)
		for d in [12, 15, 20, 30, 40, 50, 60, 80, 100]:
			addFraction(1, d)
			addFraction(d-1, d)
		data.sort()
		return data

	@staticmethod
	def fromProbability(f):
		if (f < 0 or f > 1):
			raise ValueError("probability is less than 0 or greater than 1")

		# index of the greatest element <= f
		left = bisect.bisect_right(FriendlyProbability._friendlyDescriptionValues, f)
		friendlyDescription = FriendlyProbability._friendlyDescriptionStrings[left]

		if (f == 0):
			return FriendlyProbability(0, 1, friendlyDescription)
		if (f == 1):
			return FriendlyProbability(1, 1, friendlyDescription)
		if (f > .99):
			return FriendlyProbability(99, 100, friendlyDescription, ">99 in 100")
		if (f < .01):
			return FriendlyProbability(1, 100, friendlyDescription, "<1 in 100")

		data = FriendlyProbability._fractionsData
		# index of the greatest element <= f
		left = bisect.bisect_right([x[0] for x in data], f) - 1
		if (left == len(data) - 1 or (left >= 0 and f - data[left][0] < data[left+1][0] - f)):
			return FriendlyProbability(data[left][1], data[left][2], friendlyDescription)
		else:
			return FriendlyProbability(data[left+1][1], data[left+1][2], friendlyDescription)
