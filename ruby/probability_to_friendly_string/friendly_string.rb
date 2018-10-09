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
