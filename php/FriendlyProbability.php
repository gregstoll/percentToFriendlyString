<?php
declare(strict_types=1);

final class FriendlyProbability {

    public $numerator;

    public $denominator;

    public $friendly_description;

    public $friendly_string;

    public function __construct(int $numerator, int $denominator, string $friendly_description, string $friendly_string = null) {
        $this->numerator = $numerator;
        $this->denominator = $denominator;
        $this->friendly_description = $friendly_description;
        $this->friendly_string = $friendly_string ?? ($numerator . " in " . $denominator);
    }

    public function __toString() {
        return $this->numerator . "/" . $this->denominator . " (text: \"" . $this->friendly_string . "\", description: \"" . $this->friendly_description . "\")";
    }

    private static $fractions_data = [];
	private static $friendly_description_values = [0, 0.005, 0.02, 0.08, 0.15, 0.2, 0.45, 0.55, 0.7, 0.8, 0.85, 0.9, 0.95, 0.995];
	private static $friendly_description_strings = [
		"Hard to imagine",
		"Barely possible",
		"Still possible",
		"Some chance",
		"Could happen",
		"Perhaps",
		"Flip a coin",
		"Likelier than not",
		"Good chance",
		"Probably",
		"Quite likely",
		"Pretty likely",
		"Very likely",
		"Almost certainly",
		];

    private static function get_fractions_data() {
        if (count(FriendlyProbability::$fractions_data) == 0)
        {
            $data = [];
            $add_fraction = function(array &$data, int $numerator, int $denominator) {
                $data[] = [floatval($numerator)/$denominator, $numerator, $denominator];
            };
            for ($d = 2; $d <= 10; ++$d) {
                for ($n = 1; $n < $d; ++$n) {
                    if (FriendlyProbability::gcd($n, $d) == 1) {
                        $add_fraction($data, $n, $d);
                    }
                }
            }
            foreach ([12, 15, 20, 30, 40, 50, 60, 80, 100] as $d) {
                $add_fraction($data, 1, $d);
                $add_fraction($data, $d - 1, $d);
            }
            sort($data);
            FriendlyProbability::$fractions_data = $data;
        }
        return FriendlyProbability::$fractions_data;
    }

    public static function fromProbability(float $f) {
        if ($f < 0 || $f > 1) {
            throw new DomainException("Probability is less than 0 or greater than 1");
        }
        $friendly_description_index = FriendlyProbability::binary_search(FriendlyProbability::$friendly_description_values, $f, function($a, $b) { return $a <=> $b; });
        // >= 0 means exact match, so index is correct
        if ($friendly_description_index < 0) {
            // Per the documentation, if it's not found the return value is (-insert_index - 1)
            // where insert_index is the index of smallest element that is greater than $key or sizeof($a) if $key
            // is larger than all elements in the array.
            // Here we want insert_index, so transform $index into that.
            $friendly_description_index = -1 * $friendly_description_index - 2;
        }
        $friendly_description = FriendlyProbability::$friendly_description_strings[$friendly_description_index];
        if ($f == 0) {
            return new FriendlyProbability(0, 1, $friendly_description);
        }
        if ($f == 1) {
            return new FriendlyProbability(1, 1, $friendly_description);
        }
        if ($f > .99) {
            return new FriendlyProbability(99, 100, $friendly_description, ">99 in 100");
        }
        if ($f < .01) {
            return new FriendlyProbability(1, 100, $friendly_description, "<1 in 100");
        }
        $fractions_data = FriendlyProbability::get_fractions_data();

        $index = FriendlyProbability::binary_search($fractions_data, [$f, 1000000, 1000000], function($a, $b) { return $a[0] <=> $b[0]; });
        if ($index >= 0) {
            // exact match
            return new FriendlyProbability($fractions_data[$index][1], $fractions_data[$index][2], $friendly_description);
        }
        // Per the documentation, if it's not found the return value is (-insert_index - 1)
        // where insert_index is the index of smallest element that is greater than $key or sizeof($a) if $key
        // is larger than all elements in the array.
        // Here we want insert_index, so transform $index into that.
        $next_larger_position = -1 * $index - 1;
        if ($next_larger_position == count($fractions_data)
            || ($next_larger_position - 1 >= 0 && ($f - $fractions_data[$next_larger_position - 1][0] < $fractions_data[$next_larger_position][0] - $f))) {
            return new FriendlyProbability($fractions_data[$next_larger_position - 1][1], $fractions_data[$next_larger_position - 1][2], $friendly_description);
        }
        else {
            return new FriendlyProbability($fractions_data[$next_larger_position][1], $fractions_data[$next_larger_position][2], $friendly_description);
        }
    }

    private static function gcd(int $num1, int $num2) {
        /* finds the greatest common factor between two numbers */
        while ($num2 != 0) {
            $t = $num1 % $num2;
            $num1 = $num2;
            $num2 = $t;
        }
        return $num1;
    }

    /*
     * https://terenceyim.wordpress.com/2011/02/01/all-purpose-binary-search-in-php/
     * Parameters: 
     *   $a - The sort array.
     *   $key - The key to be searched for.
     *
     * Return:
     *   index of the search key if found, otherwise return (-insert_index - 1). 
     *   insert_index is the index of smallest element that is greater than $key or sizeof($a) if $key
     *   is larger than all elements in the array.
     */
    function binary_search(array $a, $key, $compare) {
        $lo = 0;
        $hi = count($a) - 1;
 
        while ($lo <= $hi) {
            $mid = (int)(($hi - $lo) / 2) + $lo;
            $cmp = call_user_func($compare, $a[$mid], $key);
 
            if ($cmp < 0) {
                $lo = $mid + 1;
            } elseif ($cmp > 0) {
                $hi = $mid - 1;
            } else {
                return $mid;
            }
        }
        return -($lo + 1);
    }
}
