#[cfg(test)]
mod tests {
    use crate::ch07_object_oriented_design::info::*;

    #[test]
    fn chapter_info_test() {
        assert_eq!(chapter_info(), "Chapter 07: Object-Oriented Design");
    }
}
