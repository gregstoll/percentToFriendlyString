<?php
declare(strict_types=1);

final class FriendlyProbability {

    public $numerator;

    public $denominator;

    public $friendly_string;

    public function __construct(int $numerator, int $denominator, string $friendly_string = null) {
        $this->numerator = $numerator;
        $this->denominator = $denominator;
        if (is_null($friendly_string)) {
            $this->friendly_string = $numerator . " in " . $denominator;
        } else {
            $this->friendly_string = $friendly_string;
        }
    }

    public function __toString() {
        return $this->numerator . "/" . $this->denominator . " (text: \"" . $this->friendly_string . "\")";
    }

    public static function fromProbability(float $f) {
        //TODO
        return new FriendlyProbability(0, 1);
    }
}
