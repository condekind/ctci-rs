// Define the functions to check if a string contains only unique characters

pub fn is_unique_basic(input: &str) -> bool {
    use std::collections::HashSet;
    let mut chars = HashSet::new();
    input.chars().all(move |c| chars.insert(c))
}

pub fn is_unique_optimized(input: &str) -> bool {
    if input.len() > 128 {
        return false;
    }
    let mut checker = 0;
    for c in input.chars() {
        let val = c as u32 - 'a' as u32;
        if checker & (1 << val) > 0 {
            return false;
        }
        checker |= 1 << val;
    }
    true
}

pub const INPUT_EXPECTED: &[(&str, bool)] = &[("abcde", true), ("hello", false), ("", true)];
pub const LEN: i32 = INPUT_EXPECTED.len() as i32;

#[cfg(test)]
mod tests {
    use super::*;
    use gen_tests::generate_tests;

    generate_tests!(is_unique_basic, 3);
    generate_tests!(is_unique_optimized, 3);
}
