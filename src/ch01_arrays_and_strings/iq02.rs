use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct InputArgs(&'static str, &'static str);

pub fn check_permutation_hashmap(args: InputArgs) -> bool {
    // Destructure InputArgs
    let InputArgs(s0, s1) = args;

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

pub fn check_permutation_sort(args: InputArgs) -> bool {
    let InputArgs(s0, s1) = args;

    // Early return if lengths differ
    if s0.len() != s1.len() {
        return false;
    }

    // Convert &str to Vec<char> and sort
    let mut chars0: Vec<char> = s0.chars().collect();
    let mut chars1: Vec<char> = s1.chars().collect();
    chars0.sort_unstable();
    chars1.sort_unstable();

    // Compare sorted Vec<char>
    chars0 == chars1
}

pub fn check_permutation_array_count(args: InputArgs) -> bool {
    let InputArgs(s0, s1) = args;

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

pub const INPUT_EXPECTED: &[(InputArgs, bool)] = &[
    (InputArgs("", ""), true),             // Both strings are empty
    (InputArgs("abc", "abc"), true),       // Identical strings
    (InputArgs("abc", "cba"), true),       // Reversed strings
    (InputArgs("aabbcc", "abcabc"), true), // Same characters, different order
    (InputArgs("abc", "def"), false),      // Different characters
    (InputArgs("abc", "abcd"), false),     // Different lengths
    (InputArgs("aabbcc", "aabbc"), false), // Same characters, different quantities
    (InputArgs("1234", "4321"), true),     // Numbers as permutations
    (InputArgs("AbcD", "DcbA"), true),     // Mixed-case permutations
    (InputArgs("a b c", "  cba"), true),   // Strings with spaces
    (InputArgs("hello, world!", "world! hello,"), true), // Strings with punctuation
    (InputArgs("Test1!", "test1!"), false), // Different case, not permutations
    (InputArgs("A1 b2!C", "C!2b 1A"), true), // Complex case with mixed characters
];

#[cfg(test)]
mod tests {
    use super::*;
    use gen_tests::generate_tests2;

    fn eq(a: &bool, b: bool) -> bool {
        *a == b
    }

    generate_tests2!(check_permutation_hashmap, 13, eq);
    generate_tests2!(check_permutation_sort, 13, eq);
    generate_tests2!(check_permutation_array_count, 13, eq);

    generate_tests2!(check_permutation_hashmap, 13);
    generate_tests2!(check_permutation_sort, 13);
    generate_tests2!(check_permutation_array_count, 13);
}
