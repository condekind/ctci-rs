#[cfg(test)]
mod tests {
    use crate::ch16_moderate::info::*;

    #[test]
    fn chapter_info_test() {
        assert_eq!(chapter_info(), "Chapter 16: Moderate");
    }
}
