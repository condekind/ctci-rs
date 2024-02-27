#[cfg(test)]
mod tests {
    use crate::ch12_c_and_cpp::info::*;

    #[test]
    fn chapter_info_test() {
        assert_eq!(chapter_info(), "Chapter 12: C and C++");
    }
}
