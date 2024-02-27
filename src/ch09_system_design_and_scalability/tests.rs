#[cfg(test)]
mod tests {
    use crate::ch09_system_design_and_scalability::info::*;

    #[test]
    fn chapter_info_test() {
        assert_eq!(chapter_info(), "Chapter 09: System Design and Scalability");
    }
}
