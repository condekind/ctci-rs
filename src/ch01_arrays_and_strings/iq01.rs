pub fn is_unique(input: &str) -> bool {
    use std::collections::HashSet;
    let mut chars = HashSet::new();
    input.chars().all(move |c| chars.insert(c))
}

pub fn is_unique_no_ds(input: &str) -> bool {
    if input.len() > 128 {
        return false; // Since there are only 128 unique ASCII characters
    }

    let mut checker1: u64 = 0; // To hold the uniqueness status for the first 64 ASCII values
    let mut checker2: u64 = 0; // To hold the uniqueness status for the next 64 ASCII values

    for c in input.chars() {
        let val = c as u32; // Directly use ASCII value of character
        if val < 64 {
            // If the character is in the first 64 ASCII values
            if checker1 & (1 << val) > 0 {
                return false;
            }
            checker1 |= 1 << val;
        } else {
            // For characters in the next 64 ASCII values
            let val = val - 64; // Adjust to use as an index in the second checker
            if checker2 & (1 << val) > 0 {
                return false;
            }
            checker2 |= 1 << val;
        }
    }

    true
}

use lazy_static::lazy_static;

fn gen_input_expected() -> &'static [(&'static str, bool)] {
    let input_expected = &[
        ("abcde", true),
        ("hello", false),
        ("", true),
        ("Ff", true),
        ("noon", false),
        ("non", false),
        ("Non", true),
    ];

    input_expected
}

//lazy_static! {
//    static ref INPUT_EXPECTED: &'static [(&'static str, bool)] = {
//        gen_input_expected()
//    };
//}

fn gen_global_data() -> &'static [(&'static str, bool)] {
    let input_expected = &[
        ("item1", true),
        ("item2", false),
        ("item3", true),
        ("item4", false),
        ("item5", true),
    ];

    input_expected
}
lazy_static! {
    static ref GLOBAL_DATA: &'static [(&'static str, bool)] = { gen_input_expected() };
}

#[cfg(test)]
mod tests {
    use super::*;
    use gen_tests::generate_tests;

    static INPUT_EXPECTED: &[(&str, bool)] = &[
        ("abcde", true),
        ("hello", false),
        ("", true),
        ("Ff", true),
        ("noon", false),
        ("non", false),
        ("Non", true),
    ];

    //const INPUT_EXPECTED: &[(&str, bool)] = &[
    //    ("abcde", true),
    //    ("hello", false),
    //    ("", true),
    //    ("Ff", true),
    //    ("noon", false),
    //    ("non", false),
    //    ("Non", true),
    //];

    /*
    static INPUT_LEN: usize = INPUT_EXPECTED.len();
    const INPUT_LEN: usize = INPUT_EXPECTED.len();
    */

    fn eq(result: &bool, expected: bool) -> bool {
        *result == expected
    }

    // The hardcoded int literals should ideally match the length of the
    // INPUT_EXPECTED list
    generate_tests!(is_unique, 7, eq);
    generate_tests!(is_unique_no_ds, 7, eq);
}
