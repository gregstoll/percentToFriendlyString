using System;
using System.Collections.Generic;

namespace Gregstoll.ProbabilityToFriendlyString
{
    public struct FriendlyProbability : IEquatable<FriendlyProbability>
    {
        public byte Numerator { get; }
        public byte Denominator { get; }
        public string FriendlyString { get; }

        public FriendlyProbability(byte numerator, byte denominator, string friendlyString)
        {
            Numerator = numerator;
            Denominator = denominator;
            FriendlyString = friendlyString;
        }

        public FriendlyProbability(byte numerator, byte denominator) : this(numerator, denominator, string.Format("{0} in {1}", numerator, denominator))
        {
        }

        public static FriendlyProbability FromProbability(double prob)
        {
            if (prob < 0 || prob > 1)
            {
                throw new ArgumentOutOfRangeException("probability must be between 0 and 1!");
            }
            if (prob == 0)
            {
                return new FriendlyProbability(0, 1);
            }
            if (prob == 1)
            {
                return new FriendlyProbability(1, 1);
            }
            return new FriendlyProbability(1, 2);
        }

        public override bool Equals(object obj)
        {
            return obj is FriendlyProbability && Equals((FriendlyProbability)obj);
        }

        public bool Equals(FriendlyProbability other)
        {
            return Numerator == other.Numerator &&
                   Denominator == other.Denominator &&
                   FriendlyString == other.FriendlyString;
        }

        public override int GetHashCode()
        {
            var hashCode = 676277150;
            hashCode = hashCode * -1521134295 + Numerator.GetHashCode();
            hashCode = hashCode * -1521134295 + Denominator.GetHashCode();
            hashCode = hashCode * -1521134295 + EqualityComparer<string>.Default.GetHashCode(FriendlyString);
            return hashCode;
        }

        public override string ToString()
        {
            return string.Format("{0}/{1} (text: \"{2}\")", Numerator, Denominator, FriendlyString);
        }

        public static bool operator ==(FriendlyProbability probability1, FriendlyProbability probability2)
        {
            return probability1.Equals(probability2);
        }

        public static bool operator !=(FriendlyProbability probability1, FriendlyProbability probability2)
        {
            return !(probability1 == probability2);
        }
    }
}
