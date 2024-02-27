#[cfg(test)]
mod tests {
    use crate::ch03_stacks_and_queues::info::*;

    #[test]
    fn chapter_info_test() {
        assert_eq!(chapter_info(), "Chapter 03: Stacks and Queues");
    }
}
