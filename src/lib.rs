#![doc = include_str!("../README.md")]
#![deny(missing_docs, unsafe_code)]

use std::fmt::Debug;

/// Useful if the types you are debugging do not implement [`Eq`] but you want to know if they are equal
/// - Requires the type to support [`Debug`]
/// - Requires the debug output to show the part of the type you are interested in
/// - Requires the debug output to be deterministic and based on the value the type holds
pub fn eq_on_debug<T: Debug>(a: &T, b: &T) -> bool {
    let str_a = format!("{a:?}");
    let str_b = format!("{b:?}");
    str_a == str_b
}

/// Prints `second` if the debug output is not the same as that of `first`.
/// Intended usage is to print values in a loop that don't often change.
/// Returns true if it printed the value.
///
/// It uses the pretty printed version of the debug output which often spreads each value over multiple lines.
/// Color is added to lines that are not equal.
/// Expects that number of lines will be equal otherwise
/// - All unmatched lines are treated as changed
/// - Unmatched lines in `first` will not show in output, which could lead to no colored lines
pub fn print_second_if_different<T: Debug>(first: &T, second: &T) -> bool {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, 2, false)]
    #[case(1, 1, true)]
    #[case("a", "b", false)]
    #[case("apple", "apple", true)]
    fn test_equal<T: Debug>(#[case] a: T, #[case] b: T, #[case] expected: bool) {
        let actual = eq_on_debug(&a, &b);
        assert_eq!(actual, expected, "eq_on_debug failed");

        let actual = print_second_if_different(&a, &b);
        assert_eq!(actual, expected, "print_second_if_different failed");
    }
}
