#[cfg(test)]
mod tests {
    use crate::ch13_java::info::*;

    #[test]
    fn chapter_info_test() {
        assert_eq!(chapter_info(), "Chapter 13: Java");
    }
}
