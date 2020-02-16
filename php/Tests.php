<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class FriendlyProbabilityTest extends TestCase {
    public function testAllCases() {
        $file_lines = file("../testCases.txt", FILE_IGNORE_NEW_LINES);
        for ($i = 0; $i < count($file_lines); ++$i) {
            $this->line_test($i + 1, $file_lines[$i]);
        }
    }

    public function testAllFriendlyDescriptionCases() {
        $file_lines = file("../testCases.friendlyDescription.txt", FILE_IGNORE_NEW_LINES);
        for ($i = 0; $i < count($file_lines); ++$i) {
            $this->friendly_description_line_test($i + 1, $file_lines[$i]);
        }
    }


    private function line_test(int $line_number, string $line) {
        $line = trim($line);
        if (strlen($line) == 0 || substr($line, 0, 1) == "#") {
            return;
        }
        $parts = explode(",", $line);
        if (count($parts) == 3) {
            $expected = new FriendlyProbability(intval($parts[1]), intval($parts[2]), "");
        }
        else if (count($parts) == 4) {
            $expected = new FriendlyProbability(intval($parts[1]), intval($parts[2]), "", $parts[3]);
        }
        else {
            $this->assertTrue(false, "Line badly formatted: " . $line . " (line " . $line_number . ")");
        }
        $actual = FriendlyProbability::fromProbability(floatval($parts[0]));
        $this->assertEquals($expected, $actual, "line " . $line_number . " with value " . $line);

    }

    private function friendly_description_line_test(int $line_number, string $line) {
        $line = trim($line);
        if (strlen($line) == 0 || substr($line, 0, 1) == "#") {
            return;
        }
        $parts = explode(",", $line);
        $expected_friendly_description = $parts[1];
        $actual_friendly_description = FriendlyProbability::fromProbability(floatval($parts[0]))->friendly_description;
        $this->assertEquals($expected_friendly_description, $actual_friendly_description, "line " . $line_number . " with value " . $line);
    }
}
