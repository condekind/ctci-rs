#[cfg(test)]
mod tests {
    use crate::ch17_hard::info::*;

    #[test]
    fn chapter_info_test() {
        assert_eq!(chapter_info(), "Chapter 17: Hard");
    }
}
