#[cfg(test)]
mod tests {
    use crate::ch10_sorting_and_searching::info::*;

    #[test]
    fn chapter_info_test() {
        assert_eq!(chapter_info(), "Chapter 10: Sorting and Searching");
    }
}
