#[cfg(test)]
mod tests {
    use crate::ch01_arrays_and_strings::info::*;

    #[test]
    fn chapter_info_test() {
        assert_eq!(chapter_info(), "Chapter 01: Arrays and Strings");
    }
}
