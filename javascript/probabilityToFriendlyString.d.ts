class FriendlyProbability {
    constructor(numerator: number, denominator: number, friendlyString: string);

    numerator: number;
    denominator: number;
    friendlyString: string;
    friendlyDescription: string;

    toString(): string;
    equals(other: FriendlyProbability): boolean;
    static fromProbability(f: number): FriendlyProbability;
}
export = FriendlyProbability;
