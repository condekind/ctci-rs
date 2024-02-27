#[cfg(test)]
mod tests {
    use crate::ch06_math_and_logic_puzzles::info::*;

    #[test]
    fn chapter_info_test() {
        assert_eq!(chapter_info(), "Chapter 06: Math and Logic Puzzles");
    }
}
