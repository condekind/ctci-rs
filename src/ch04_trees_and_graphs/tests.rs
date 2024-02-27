#[cfg(test)]
mod tests {
    use crate::ch04_trees_and_graphs::info::*;

    #[test]
    fn chapter_info_test() {
        assert_eq!(chapter_info(), "Chapter 04: Trees and Graphs");
    }
}
