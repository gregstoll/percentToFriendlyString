<?php
declare(strict_types=1);

final class FriendlyProbability {

    public $numerator;

    public $denominator;

    public $friendly_string;

    public function __construct(int $numerator, int $denominator, string $friendly_string = null) {
        $this->numerator = $numerator;
        $this->denominator = $denominator;
        $this->friendly_string = $friendly_string ?? ($numerator . " in " . $denominator);
    }

    public function __toString() {
        return $this->numerator . "/" . $this->denominator . " (text: \"" . $this->friendly_string . "\")";
    }

    private static $fractions_data = [];
    private static function get_fractions_data() {
        if (count(FriendlyProbability::$fractions_data) == 0)
        {
            $data = [];
            $add_fraction = function(array &$data, int $numerator, int $denominator) {
                $data[] = [floatval($numerator)/$denominator, $numerator, $denominator];
            };
            for ($d = 2; $d <= 11; ++$d) {
                for ($n = 1; $n < $d; ++$n) {
                    if (FriendlyProbability::gcd($n, $d) == 1) {
                        $add_fraction($data, $n, $d);
                    }
                }
            }
            //TODO more
            sort($data);
            FriendlyProbability::$fractions_data = $data;
        }
        return FriendlyProbability::$fractions_data;
    }

    public static function fromProbability(float $f) {
        if ($f < 0 || $f > 1) {
            throw new DomainException("Probability is less than 0 or greater than 1");
        }
        if ($f == 0) {
            return new FriendlyProbability(0, 1);
        }
        if ($f == 1) {
            return new FriendlyProbability(1, 1);
        }
        if ($f > .99) {
            return new FriendlyProbability(99, 100, ">99 in 100");
        }
        if ($f < .01) {
            return new FriendlyProbability(1, 100, "<1 in 100");
        }
        $fractions_data = FriendlyProbability::get_fractions_data();

        $index = FriendlyProbability::binary_search($fractions_data, [$f, 1000000, 1000000], function($a, $b) { return $a <=> $b; });
        if ($index >= 0) {
            // exact match
            return new FriendlyProbability($fractions_data[$index][1], $fractions_data[$index][2]);
        }
        // Per the documentation, if it's not found the return value is the bitwise complement
        // of the index of the next element that's larger than the item.
        $next_larger_position = ~$index;
        if ($next_larger_position == count($fractions_data)
            || ($next_larger_position - 1 >= 0 && ($f - $fractions_data[$next_larger_position - 1][0] < $fractions_data[$next_larger_position][0] - $f))) {
            return new FriendlyProbability($fractions_data[$next_larger_position - 1][1], $fractions_data[$next_larger_position - 1][2]);
        }
        else {
            return new FriendlyProbability($fractions_data[$next_larger_position][1], $fractions_data[$next_larger_position][2]);
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
