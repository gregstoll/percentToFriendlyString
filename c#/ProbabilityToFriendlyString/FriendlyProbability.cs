using System;
using System.Collections.Generic;

namespace Gregstoll.ProbabilityToFriendlyString
{
    public struct FriendlyProbability : IEquatable<FriendlyProbability>
    {
        public byte Numerator { get; }
        public byte Denominator { get; }
        public string FriendlyDescription { get; }
        public string FriendlyString { get; }

        public FriendlyProbability(byte numerator, byte denominator, string friendlyDescription, string friendlyString)
        {
            Numerator = numerator;
            Denominator = denominator;
            FriendlyDescription = friendlyDescription;
            FriendlyString = friendlyString;
        }

        public FriendlyProbability(byte numerator, byte denominator, string friendlyDescription) :
            this(numerator, denominator, friendlyDescription, string.Format("{0} in {1}", numerator, denominator))
        {
        }

        private static Lazy<List<Tuple<double, byte, byte>>> _fractionData = new Lazy<List<Tuple<double, byte, byte>>>(() =>
        {
            var data = new List<Tuple<double, byte, byte>>();

            void AddFraction(byte numerator, byte denominator)
            {
                data.Add(Tuple.Create((double)numerator / denominator, numerator, denominator));
            }

            bool AreRelativelyPrime(byte n1, byte n2)
            {
                int Gcd(int a, int b)
                {
                    int t = 0;
                    while (b != 0)
                    {
                        t = a;
                        a = b;
                        b = t % b;
                    }
                    return a;
                }
                return Gcd(n1, n2) == 1;
            }

            for (byte d = 2; d <= 10; ++d)
            {
                for (byte n = 1; n < d; ++n)
                {
                    if (AreRelativelyPrime(n, d))
                    {
                        AddFraction(n, d);
                    }
                }
            }
            foreach (byte b in new byte[] { 12, 15, 20, 30, 40, 50, 60, 80, 100 })
            {
                AddFraction(1, b);
                AddFraction((byte)(b - 1), b);
            }

            data.Sort();
            return data;
        });

        public static FriendlyProbability FromProbability(double prob)
        {
            if (prob < 0 || prob > 1)
            {
                throw new ArgumentOutOfRangeException("probability must be between 0 and 1!");
            }
            //TODO
            string friendlyDescription = string.Empty;
            if (prob == 0)
            {
                return new FriendlyProbability(0, 1, friendlyDescription);
            }
            if (prob == 1)
            {
                return new FriendlyProbability(1, 1, friendlyDescription);
            }
            if (prob > 0.99)
            {
                return new FriendlyProbability(99, 100, friendlyDescription, ">99 in 100");
            }
            if (prob < 0.01)
            {
                return new FriendlyProbability(1, 100, friendlyDescription, "<1 in 100");
            }

            var data = _fractionData.Value;
            int position = data.BinarySearch(Tuple.Create(prob, (byte)0, (byte)0));
            if (position >= 0)
            {
                // exact match
                return new FriendlyProbability(data[position].Item2, data[position].Item3, friendlyDescription);
            }
            // Per the documentation, if it's not found the return value is the bitwise complement
            // of the index of the next element that's larger than the item.
            int nextLargerPosition = ~position;
            if (nextLargerPosition == data.Count
                || (nextLargerPosition - 1 >= 0
                    && prob - data[nextLargerPosition - 1].Item1 < data[nextLargerPosition].Item1 - prob))
            {
                return new FriendlyProbability(data[nextLargerPosition - 1].Item2, data[nextLargerPosition - 1].Item3, friendlyDescription);
            }
            else
            {
                return new FriendlyProbability(data[nextLargerPosition].Item2, data[nextLargerPosition].Item3, friendlyDescription);
            }
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
