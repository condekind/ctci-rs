#[cfg(test)]
mod tests {
    use crate::ch15_threads_and_locks::info::*;

    #[test]
    fn chapter_info_test() {
        assert_eq!(chapter_info(), "Chapter 15: Threads and Locks");
    }
}
