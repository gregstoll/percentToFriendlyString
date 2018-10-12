module ProbabilityToFriendlyString
    class FriendlyString
        include Comparable
        attr_reader :numerator, :denominator, :friendlyString

        @@fractionsData = []
        def initialize(numerator, denominator, friendlyString = nil)
            @numerator = numerator
            @denominator = denominator
            if friendlyString
                @friendlyString = friendlyString 
            else
                @friendlyString = "%d in %d" % [numerator, denominator]
            end
        end

        def self.fromProbability(f)
            if f < 0 or f > 1
                raise RangeError, "f is less than 0 or greater than 1"
            end
            if f == 0
                return FriendlyString.new 0, 1
            elsif f == 1
                return FriendlyString.new 1, 1
            elsif f > 0.99
                return FriendlyString.new 99, 100, ">99 in 100"
            elsif f < 0.01
                return FriendlyString.new 1, 100, "<1 in 100"
            end

            return FriendlyString.new 0, 1
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
