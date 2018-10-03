package com.gregstoll.probabilityToFriendlyString;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Objects;
import java.math.BigInteger;

public class FriendlyProbability {
    private byte _numerator;
    private byte _denominator;
    private String _friendlyString;

    public FriendlyProbability(byte numerator, byte denominator, String friendlyString)
    {
        _numerator = numerator;
        _denominator = denominator;
        _friendlyString = friendlyString;
    }

    public FriendlyProbability(byte numerator, byte denominator) {
        this(numerator, denominator, String.format("%d in %d", numerator, denominator));
    }

    private static class Fraction implements Comparable<Fraction> {
        private double _value;
        private byte _numerator;
        private byte _denominator;

        public Fraction(byte numerator, byte denominator) {
            _value = (double)numerator/denominator;
            _numerator = numerator;
            _denominator = denominator;
        }

        @Override
        public int compareTo(Fraction other) {
            return new Double(_value).compareTo(new Double(other._value));
        }

        @Override
        public String toString() {
            return String.format("%d/%d", _numerator, _denominator);
        }

        public double getValue() {
            return _value;
        }
        public byte getNumerator() {
            return _numerator;
        }
        public byte getDenominator() {
            return _denominator;
        }
    }

    private static ArrayList<Fraction> _fractions = null;

    private static void populateFractionsIfNecessary() {
        if (_fractions == null) {
            _fractions = new ArrayList<Fraction>();
            for (byte d = 2; d <= 10; ++d) {
                for (byte n = 1; n < d; ++n) {
                    if (BigInteger.valueOf(d).gcd(BigInteger.valueOf(n)).intValue() == 1) {
                        _fractions.add(new Fraction(n, d));
                    }
                }
            }
            for (byte b : new byte[] {12, 15, 20, 30, 40, 50, 60, 80, 100}) {
                _fractions.add(new Fraction((byte)1, b));
                _fractions.add(new Fraction((byte)(b - 1), b));
            }
            Collections.sort(_fractions);
        }
    }

    private static int binarySearch(double prob) {
        int lo = -1, hi = _fractions.size();
        while (1 + lo < hi) {
            int mi = lo + ((hi - lo) >> 1);
            if (prob <= _fractions.get(mi).getValue()) {
                hi = mi;
            }
            else {
                lo = mi;
            }
        }
        return hi;
    }

    public static FriendlyProbability fromProbability(double prob) {
        if (prob < 0 || prob > 1) {
            throw new IllegalArgumentException("probability must be between 0 and 1!");
        }
        if (prob == 0) {
            return new FriendlyProbability((byte)0, (byte)1);
        }
        if (prob == 1) {
            return new FriendlyProbability((byte)1, (byte)1);
        }
        if (prob > 0.99) {
            return new FriendlyProbability((byte)99, (byte)100, ">99 in 100");
        }
        if (prob < 0.01) {
            return new FriendlyProbability((byte)1, (byte)100, "<1 in 100");
        }
        populateFractionsIfNecessary();
        int right = binarySearch(prob);
        int left = right - 1;
        Fraction fraction;
        if (left >= 0 && prob - _fractions.get(left).getValue() < _fractions.get(right).getValue() - prob) {
            fraction = _fractions.get(left);
        }
        else {
            fraction = _fractions.get(right);
        }
        return new FriendlyProbability(fraction.getNumerator(), fraction.getDenominator());
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) {
            return true;
        }
        if (o == null) {
            return false;
        }
        if (getClass() != o.getClass()) {
            return false;
        }
        FriendlyProbability otherProb = (FriendlyProbability)o;
        return _numerator == otherProb._numerator && _denominator == otherProb._denominator && _friendlyString.equals(otherProb._friendlyString);
    }

    @Override
    public int hashCode() {
        return Objects.hash(_numerator, _denominator, _friendlyString);
    }

    @Override
    public String toString() {
        return String.format("%d/%d (text: \"%s\")", _numerator, _denominator, _friendlyString);
    }

    public byte getNumerator() {
        return _numerator;
    }

    public byte getDenominator() {
        return _denominator;
    }

    public String getFriendlyString() {
        return _friendlyString;
    }
}
