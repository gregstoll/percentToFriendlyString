require 'test/unit'
require './friendly_string'

class FriendlyStringTests < Test::Unit::TestCase
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
                expected = ProbabilityToFriendlyString::FriendlyString.new parts[1].to_i, parts[2].to_i
            when 4
                expected = ProbabilityToFriendlyString::FriendlyString.new parts[1].to_i, parts[2].to_i, parts[3]
            else
                flunk "Line badly formatted: #{line} (line #{lineNumber})"
            end
            actual = ProbabilityToFriendlyString::FriendlyString.fromProbability(parts[0].to_f)
            assert_equal expected, actual, "Called on #{parts[0]} (line #{lineNumber})"
        end
    end
end
