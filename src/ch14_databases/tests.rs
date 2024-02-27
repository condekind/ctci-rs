#[cfg(test)]
mod tests {
    use crate::ch14_databases::info::*;

    #[test]
    fn chapter_info_test() {
        assert_eq!(chapter_info(), "Chapter 14: Databases");
    }
}
