
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

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;

    const INPUT_EXPECTED: &[(&str, bool)] = &[
        ("abcde", true),
        ("hello", false),
        ("", true),
    ];
    const INPUT_LEN: usize = INPUT_EXPECTED.len();

    macro_rules! generate_test {
        ($func:ident, $index:expr) => {
            paste! {
                #[test]
                fn [<test_ $func _ $index>]() {
                    let (input, expected) = INPUT_EXPECTED[$index];
                    let result = $func(input);
                    assert!(result == expected, "{} failed on input: {:?}", stringify!($func), input);
                }
            }
        };
    }

    macro_rules! generate_tests {
        ($($func:ident),+) => {
            $(
                generate_test!($func, 0);
                generate_test!($func, 1);
                generate_test!($func, 2);
            )+
        };
    }

    generate_tests!(is_unique_basic, is_unique_optimized);

}