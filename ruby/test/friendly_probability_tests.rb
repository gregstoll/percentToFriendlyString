require 'test/unit'
require_relative '../lib/probability_to_friendly_string'

class FriendlyProbabilityTests < Test::Unit::TestCase
    def test_all_cases
        directory = File.absolute_path(File.dirname(__FILE__))
        path = File.join(directory, 'testCases.txt')
        while !File.exists?(path)
            directory = File.dirname(directory)
            path = File.join(directory, 'testCases.txt')
        end
        lineNumber = 0
        File.open(path).each do |fullLine|
            lineNumber += 1
            line = fullLine.strip
            if (line.start_with?('#') or line.empty?)
                next
            end
            parts = line.split(',')
            case parts.length
            when 3
                expected = ProbabilityToFriendlyString::FriendlyProbability.new parts[1].to_i, parts[2].to_i, nil
            when 4
                expected = ProbabilityToFriendlyString::FriendlyProbability.new parts[1].to_i, parts[2].to_i, nil, parts[3]
            else
                flunk "Line badly formatted: #{line} (line #{lineNumber})"
            end
            actual = ProbabilityToFriendlyString::FriendlyProbability.fromProbability(parts[0].to_f)
            assert_equal expected.numerator, actual.numerator, "Numerator, called on #{parts[0]} (line #{lineNumber})"
            assert_equal expected.denominator, actual.denominator, "Denominator, called on #{parts[0]} (line #{lineNumber})"
            assert_equal expected.friendlyString, actual.friendlyString, "Denominator, called on #{parts[0]} (line #{lineNumber})"
        end
    end
end
