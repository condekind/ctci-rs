use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;

fn main() {
    let chapters = [
        ("ch01_arrays_and_strings", "Chapter 01: Arrays and Strings"),
        ("ch02_linked_lists", "Chapter 02: Linked Lists"),
        ("ch03_stacks_and_queues", "Chapter 03: Stacks and Queues"),
        ("ch04_trees_and_graphs", "Chapter 04: Trees and Graphs"),
        ("ch05_bit_manipulation", "Chapter 05: Bit Manipulation"),
        (
            "ch06_math_and_logic_puzzles",
            "Chapter 06: Math and Logic Puzzles",
        ),
        (
            "ch07_object_oriented_design",
            "Chapter 07: Object-Oriented Design",
        ),
        (
            "ch08_recursion_and_dynamic_programming",
            "Chapter 08: Recursion and Dynamic Programming",
        ),
        (
            "ch09_system_design_and_scalability",
            "Chapter 09: System Design and Scalability",
        ),
        (
            "ch10_sorting_and_searching",
            "Chapter 10: Sorting and Searching",
        ),
        ("ch11_testing", "Chapter 11: Testing"),
        ("ch12_c_and_cpp", "Chapter 12: C and C++"),
        ("ch13_java", "Chapter 13: Java"),
        ("ch14_databases", "Chapter 14: Databases"),
        ("ch15_threads_and_locks", "Chapter 15: Threads and Locks"),
        ("ch16_moderate", "Chapter 16: Moderate"),
        ("ch17_hard", "Chapter 17: Hard"),
    ];

    let src_path = Path::new("src");
    if !src_path.exists() {
        fs::create_dir_all(src_path).expect("Failed to create src directory");
    }

    for (dir_name, chapter_title) in chapters.iter() {
        let chapter_path = src_path.join(dir_name);
        if !chapter_path.exists() {
            fs::create_dir_all(&chapter_path).expect("Failed to create chapter directory");
        }

        // info.rs
        let info_path = chapter_path.join("info.rs");
        let mut info_file = File::create(&info_path).expect("Failed to create info file");
        writeln!(
            info_file,
            "pub fn chapter_info() -> &'static str {{ \"{}\" }}",
            chapter_title
        )
        .expect("Failed to write to info file");

        // tests.rs
        let tests_path = chapter_path.join("tests.rs");
        let mut tests_file = File::create(&tests_path).expect("Failed to create tests file");
        writeln!(
            tests_file,
            r#"#[cfg(test)]
mod tests {{
    use crate::{}::info::*;

    #[test]
    fn chapter_info_test() {{
        assert_eq!(chapter_info(), "{}");
    }}
}}"#,
            dir_name, chapter_title,
        )
        .expect("Failed to write to tests file");

        // mod.rs
        let mod_path = chapter_path.join("mod.rs");
        if !mod_path.exists() {
            let mut mod_file = File::create(&mod_path).expect("Failed to create mod file");
            writeln!(mod_file, "pub mod info;\nmod tests;").expect("Failed to write to mod file");
        }
    }

    // lib.rs
    let lib_path = src_path.join("lib.rs");
    if !lib_path.exists() {
        fs::create_dir_all(&lib_path).expect("Failed to create chapter directory");
        let mut lib_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&lib_path)
            .expect("Failed to open lib.rs");
        for (dir_name, _) in chapters.iter() {
            writeln!(lib_file, "pub mod {};", dir_name).expect("Failed to write to lib.rs");
        }
    }
}
