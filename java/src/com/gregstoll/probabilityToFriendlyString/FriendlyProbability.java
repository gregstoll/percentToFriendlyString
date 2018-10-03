package com.gregstoll.probabilityToFriendlyString;
import java.util.Objects;

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

    public static FriendlyProbability fromProbability(double prob) {
        //TODO implement this
        return new FriendlyProbability((byte)0, (byte)1);
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
