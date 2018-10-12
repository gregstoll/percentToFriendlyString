module ProbabilityToFriendlyString
    class FriendlyProbability
        include Comparable
        attr_reader :numerator, :denominator, :friendlyString

        @@fractionsData = nil
        def initialize(numerator, denominator, friendlyString = nil)
            @numerator = numerator
            @denominator = denominator
            if friendlyString
                @friendlyString = friendlyString 
            else
                @friendlyString = "%d in %d" % [numerator, denominator]
            end
        end

        def self._addFraction(fractionsData, numerator, denominator)
            fractionsData << [numerator.to_f/denominator, numerator, denominator]
        end
        def self._createFractionsData
            if @@fractionsData
                return
            end
            fractionsData = []
            (2..10).each do |d|
                (1..d).each do |n|
                    if n.gcd(d) == 1
                        _addFraction(fractionsData, n, d)
                    end
                end
            end
            [12, 15, 20, 30, 40, 50, 60, 80, 100].each do |d|
                _addFraction(fractionsData, 1, d)
                _addFraction(fractionsData, d - 1, d)
            end
            @@fractionsData = fractionsData.sort
        end

        def self.fromProbability(f)
            if f < 0 or f > 1
                raise RangeError, "f is less than 0 or greater than 1"
            end
            if f == 0
                return FriendlyProbability.new 0, 1
            elsif f == 1
                return FriendlyProbability.new 1, 1
            elsif f > 0.99
                return FriendlyProbability.new 99, 100, ">99 in 100"
            elsif f < 0.01
                return FriendlyProbability.new 1, 100, "<1 in 100"
            end

            FriendlyProbability._createFractionsData
            # index of the least element > f
            right = @@fractionsData.bsearch_index {|x| x[0] > f}
            if right
                left = right - 1
            else
                left = @@fractionsData.length - 1
            end
            if (left == (@@fractionsData.length - 1) or (left >= 0 and f - @@fractionsData[left][0] < @@fractionsData[right][0] - f))
                return FriendlyProbability.new @@fractionsData[left][1], @@fractionsData[left][2]
            else
                return FriendlyProbability.new @@fractionsData[right][1], @@fractionsData[right][2]
            end
        end

        def to_s
            return "#{@numerator}/#{@denominator} (text: \"#{@friendlyString}\")"
        end

        def <=>(another_friendly_string)
            if self.numerator < another_friendly_string.numerator
                -1
            elsif self.numerator > another_friendly_string.numerator
                1
            end
            if self.denominator < another_friendly_string.denominator
                -1
            elsif self.denominator > another_friendly_string.denominator
                1
            end
            if self.friendlyString < another_friendly_string.friendlyString
                -1
            elsif self.friendlyString > another_friendly_string.friendlyString
                1
            else
                0
            end
        end
    end
end
