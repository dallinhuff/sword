/// An indicator of whether a letter in a guess is in the word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Placement {
    /// The letter is not in the solution.
    Incorrect,

    /// The letter is in the solution, but not in the given spot.
    Misplaced,

    /// The letter is in the solution in the given spot.
    Correct,
}

#[cfg(test)]
mod tests {
    use super::*;

    // I know this test is stupid, but I want to make it explicit we're relying on derived Ord's
    // behavior of making later discriminants > earlier discriminants
    #[test]
    fn ord_works() {
        let assertions = [
            Placement::Incorrect < Placement::Misplaced,
            Placement::Incorrect < Placement::Correct,
            Placement::Misplaced < Placement::Correct,
            Placement::Correct > Placement::Misplaced,
            Placement::Correct > Placement::Incorrect,
            Placement::Misplaced > Placement::Incorrect,
            Placement::Incorrect == Placement::Incorrect,
            Placement::Misplaced == Placement::Misplaced,
            Placement::Correct == Placement::Correct,
        ];

        for assertion in assertions {
            assert!(assertion);
        }
    }
}
