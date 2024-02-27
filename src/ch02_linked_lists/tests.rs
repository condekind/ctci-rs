#[cfg(test)]
mod tests {
    use crate::ch02_linked_lists::info::*;

    #[test]
    fn chapter_info_test() {
        assert_eq!(chapter_info(), "Chapter 02: Linked Lists");
    }
}
