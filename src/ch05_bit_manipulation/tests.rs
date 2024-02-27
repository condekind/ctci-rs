#[cfg(test)]
mod tests {
    use crate::ch05_bit_manipulation::info::*;

    #[test]
    fn chapter_info_test() {
        assert_eq!(chapter_info(), "Chapter 05: Bit Manipulation");
    }
}
