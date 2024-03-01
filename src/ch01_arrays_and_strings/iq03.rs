use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct Params(pub &'static str, pub &'static str);

fn check_permutation_hashmap(args: Params) -> bool {
    let Params(s0, s1) = args;

    if s0.len() != s1.len() {
        return false;
    }

    let mut cnt0 = HashMap::new();
    for c in s0.chars() {
        *cnt0.entry(c).or_insert(0) += 1;
    }

    let mut cnt1 = HashMap::new();
    for c in s1.chars() {
        *cnt1.entry(c).or_insert(0) += 1;
    }

    cnt0.eq(&cnt1)
}

fn check_permutation_sort(args: Params) -> bool {
    let Params(s0, s1) = args;

    if s0.len() != s1.len() {
        return false;
    }

    let mut chars0: Vec<char> = s0.chars().collect();
    let mut chars1: Vec<char> = s1.chars().collect();
    chars0.sort_unstable();
    chars1.sort_unstable();

    chars0 == chars1
}

pub fn check_permutation_array_count(args: Params) -> bool {
    let Params(s0, s1) = args;

    // Early return if lengths differ
    if s0.len() != s1.len() {
        return false;
    }

    // Assuming ASCII characters, create a fixed-size array for counts
    let mut counts = [0i32; 128]; // ASCII has 128 characters

    // Count the occurrences of each character in s0
    for byte in s0.as_bytes() {
        counts[*byte as usize] += 1;
    }

    // Subtract the counts for characters in s1
    for byte in s1.as_bytes() {
        counts[*byte as usize] -= 1;
        // If any count goes negative, s1 has more of that char than s0
        if counts[*byte as usize] < 0 {
            return false;
        }
    }

    // If all counts are zero, s1 is a permutation of s0
    true
}

pub const INPUT_EXPECTED: &[(Params, bool)] = &[
    (Params("", ""), true),                           // Both strings are empty
    (Params("abc", "abc"), true),                     // Identical strings
    (Params("abc", "cba"), true),                     // Reversed strings
    (Params("aabbcc", "abcabc"), true),               // Same characters, different order
    (Params("abc", "def"), false),                    // Different characters
    (Params("abc", "abcd"), false),                   // Different lengths
    (Params("aabbcc", "aabbc"), false),               // Same characters, different quantities
    (Params("1234", "4321"), true),                   // Numbers as permutations
    (Params("AbcD", "DcbA"), true),                   // Mixed-case permutations
    (Params("a b c", "  cba"), true),                 // Strings with spaces
    (Params("hello, world!", "world! hello,"), true), // Strings with punctuation
    (Params("Test1!", "test1!"), false),              // Different case, not permutations
    (Params("A1 b2!C", "C!2b 1A"), true),             // Complex case with mixed characters
];

#[cfg(test)]
mod tests {
    use super::*;
    use extra::extra::*;
    use gen_tests::generate_tests2;

    fn eq(a: &bool, b: bool) -> bool {
        *a == b
    }

    //generate_tests2!(check_permutation_hashmap, 13, eq);
    //generate_tests2!(check_permutation_sort, 13, eq);
    //generate_tests2!(check_permutation_array_count, 13, eq);

    #[test]
    fn test_permutation_hashmap_default_check() {
        let checker = Solver::with_default_check(check_permutation_hashmap);

        for (input, expected) in INPUT_EXPECTED.iter() {
            let result = checker.solve(*input);
            assert!(
                checker.check_output(expected, result),
                "Failed check_permutation_hashmap on input: {:?}",
                input
            );
        }
    }

    #[test]
    fn test_permutation_sort_default_check() {
        let checker = Solver::with_default_check(check_permutation_sort);

        for (input, expected) in INPUT_EXPECTED.iter() {
            let result = checker.solve(*input);
            assert!(
                checker.check_output(expected, result),
                "Failed check_permutation_sort on input: {:?}",
                input
            );
        }
    }

    #[test]
    fn test_permutation_hashmap_custom_check() {
        let checker = Solver::with_custom_check(check_permutation_hashmap, eq);

        for (input, expected) in INPUT_EXPECTED.iter() {
            let result = checker.solve(*input);
            assert!(
                checker.check_output(expected, result),
                "Failed check_permutation_hashmap on input: {:?}",
                input
            );
        }
    }

    #[test]
    fn test_permutation_sort_custom_check() {
        let checker = Solver::with_custom_check(check_permutation_sort, eq);

        for (input, expected) in INPUT_EXPECTED.iter() {
            let result = checker.solve(*input);
            assert!(
                checker.check_output(expected, result),
                "Failed check_permutation_sort on input: {:?}",
                input
            );
        }
    }
}
