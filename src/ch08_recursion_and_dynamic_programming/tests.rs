#[cfg(test)]
mod tests {
    use crate::ch08_recursion_and_dynamic_programming::info::*;

    #[test]
    fn chapter_info_test() {
        assert_eq!(
            chapter_info(),
            "Chapter 08: Recursion and Dynamic Programming"
        );
    }
}
