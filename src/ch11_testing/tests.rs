#[cfg(test)]
mod tests {
    use crate::ch11_testing::info::*;

    #[test]
    fn chapter_info_test() {
        assert_eq!(chapter_info(), "Chapter 11: Testing");
    }
}
