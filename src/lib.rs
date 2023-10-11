#![doc = include_str!("../README.md")]
#![deny(missing_docs, unsafe_code)]

use colored::Colorize;
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
/// Returns true if values are the same and nothing was printed.
/// Otherwise it prints and returns false.
///
/// It uses the pretty printed version of the debug output which often spreads each value over multiple lines.
/// Color is added to lines that are not equal.
/// Expects that number of lines will be equal otherwise
/// - All unmatched lines are treated as changed
/// - Unmatched lines in `first` will not show in output, which could lead to no colored lines
pub fn print_second_if_different<T: Debug>(first: &T, second: &T, color: colored::Color) -> bool {
    let first = format!("{first:#?}");
    let second = format!("{second:#?}");
    if first == second {
        true
    } else {
        let first: Vec<_> = first.lines().collect();
        let second: Vec<_> = second.lines().collect();
        for (i, second_line) in second.iter().enumerate() {
            let is_diff = if let Some(first_line) = first.get(i) {
                first_line != &second[i]
            } else {
                true
            };

            if is_diff {
                println!("{}", second_line.color(color));
            } else {
                println!("{second_line}");
            }
        }
        false
    }
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
