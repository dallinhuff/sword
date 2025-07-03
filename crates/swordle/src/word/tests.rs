use super::*;

fn run_constructor_tests(cases: &[&str], expected: &WordError) {
    for input in cases {
        assert_eq!(Word::new(input).as_ref(), Err(expected));
    }
}

#[test]
fn constructor_returns_letter_error_when_invalid_letter() {
    let test_cases = ["cråne", "sh-me", "cr4bs"];
    run_constructor_tests(&test_cases, &WordError::Letter);
}

#[test]
fn constructor_returns_length_error_when_invalid_len() {
    let test_cases = ["automobile", "ego", "", " base", "some\t"];
    run_constructor_tests(&test_cases, &WordError::Length);
}

#[test]
fn constructor_returns_bank_error_when_invalid_word() {
    let test_cases = ["asdfg", "crune", "falst"];
    run_constructor_tests(&test_cases, &WordError::Bank);
}
