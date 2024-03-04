use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

extern crate chrono;
use chrono::Utc;
use syn::{self, visit::Visit, ExprArray, ItemConst, ItemStatic};

struct SliceVisitor {
    // This tracks information during the AST traversal.
    // One of the reasons we need `inside_item` is because we define the trait
    // method `visit_expr_array` - which means it will run unconditionally for
    // all `ExprArray`s of a target file. But we only want to execute the code
    // of our defined `visit_expr_array` when we call it directly from the
    // defined `visit_item_const`
    //

    // Stores:
    // 0. Whether the Visitor is inside an ItemStatic (not necessarily
    // directly, it could be in an Expr, but 'true' would mean the Expr is
    // inside an ItemStatic)
    // 1. The Ident (name) of the Item the Visitor is in. This is used in
    // accesses to the `slices` field
    inside_item: (bool, String),

    // Keeps track of which file is currently being parsed
    file_name: String,

    // Stores tuples to track all static slices (idents) and their lengths
    slices: HashMap<String, usize>,
}

impl<'ast> Visit<'ast> for SliceVisitor {
    fn visit_expr_array(&mut self, expr: &'ast ExprArray) {
        // We only do anything if we arrived here through `visit_item_check`
        if self.inside_item.0 {
            self.slices
                .insert(self.inside_item.1.clone(), expr.elems.len());
            println!("Length of {}: {}", self.inside_item.1, expr.elems.len());
        }
    }

    fn visit_item_const(&mut self, item: &'ast ItemConst) {
        if let syn::Expr::Reference(expr_ref) = &*item.expr {
            if let syn::Expr::Array(expr_array) = &*expr_ref.expr {
                self.inside_item = (true, item.ident.to_string());
                self.visit_expr_array(expr_array);
                self.inside_item = (false, String::from(""));
            }
        }
    }

    fn visit_item_static(&mut self, item: &'ast ItemStatic) {
        if let syn::Expr::Reference(expr_ref) = &*item.expr {
            if let syn::Expr::Array(expr_array) = &*expr_ref.expr {
                self.inside_item = (true, item.ident.to_string());
                self.visit_expr_array(expr_array);
                self.inside_item = (false, String::from(""));
            }
        }
    }
}

pub fn main() -> io::Result<()> {
    // Root of the project
    let root = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| String::from("."));

    // All files that the SliceVisitor will consider when searching for const
    // or static slices
    let files = {
        vec![
            "/src/ch01_arrays_and_strings/iq01.rs",
            "/src/ch01_arrays_and_strings/iq02.rs",
            "/src/ch01_arrays_and_strings/iq03.rs",
            "/src/ch01_arrays_and_strings/iq04.rs",
            "/src/ch01_arrays_and_strings/iq05.rs",
            "/src/ch01_arrays_and_strings/iq06.rs",
            "/src/ch01_arrays_and_strings/iq07.rs",
            "/src/ch01_arrays_and_strings/iq08.rs",
            "/src/ch01_arrays_and_strings/iq09.rs",
            "/src/ch02_linked_lists/iq01.rs",
            "/src/ch02_linked_lists/iq02.rs",
            "/src/ch02_linked_lists/iq03.rs",
            "/src/ch02_linked_lists/iq04.rs",
            "/src/ch02_linked_lists/iq05.rs",
            "/src/ch02_linked_lists/iq06.rs",
            "/src/ch02_linked_lists/iq07.rs",
            "/src/ch02_linked_lists/iq08.rs",
            "/src/ch03_stacks_and_queues/iq01.rs",
            "/src/ch03_stacks_and_queues/iq02.rs",
            "/src/ch03_stacks_and_queues/iq03.rs",
            "/src/ch03_stacks_and_queues/iq04.rs",
            "/src/ch03_stacks_and_queues/iq05.rs",
            "/src/ch03_stacks_and_queues/iq06.rs",
            "/src/ch04_trees_and_graphs/iq01.rs",
            "/src/ch04_trees_and_graphs/iq02.rs",
            "/src/ch04_trees_and_graphs/iq03.rs",
            "/src/ch04_trees_and_graphs/iq04.rs",
            "/src/ch04_trees_and_graphs/iq05.rs",
            "/src/ch04_trees_and_graphs/iq06.rs",
            "/src/ch04_trees_and_graphs/iq07.rs",
            "/src/ch04_trees_and_graphs/iq08.rs",
            "/src/ch04_trees_and_graphs/iq09.rs",
            "/src/ch04_trees_and_graphs/iq10.rs",
            "/src/ch04_trees_and_graphs/iq11.rs",
            "/src/ch04_trees_and_graphs/iq12.rs",
            "/src/ch05_bit_manipulation/iq01.rs",
            "/src/ch05_bit_manipulation/iq02.rs",
            "/src/ch05_bit_manipulation/iq03.rs",
            "/src/ch05_bit_manipulation/iq04.rs",
            "/src/ch05_bit_manipulation/iq05.rs",
            "/src/ch05_bit_manipulation/iq06.rs",
            "/src/ch05_bit_manipulation/iq07.rs",
            "/src/ch05_bit_manipulation/iq08.rs",
            "/src/ch06_math_and_logic_puzzles/iq01.rs",
            "/src/ch06_math_and_logic_puzzles/iq02.rs",
            "/src/ch06_math_and_logic_puzzles/iq03.rs",
            "/src/ch06_math_and_logic_puzzles/iq04.rs",
            "/src/ch06_math_and_logic_puzzles/iq05.rs",
            "/src/ch06_math_and_logic_puzzles/iq06.rs",
            "/src/ch06_math_and_logic_puzzles/iq07.rs",
            "/src/ch06_math_and_logic_puzzles/iq08.rs",
            "/src/ch06_math_and_logic_puzzles/iq09.rs",
            "/src/ch06_math_and_logic_puzzles/iq10.rs",
            "/src/ch07_object_oriented_design/iq01.rs",
            "/src/ch07_object_oriented_design/iq02.rs",
            "/src/ch07_object_oriented_design/iq03.rs",
            "/src/ch07_object_oriented_design/iq04.rs",
            "/src/ch07_object_oriented_design/iq05.rs",
            "/src/ch07_object_oriented_design/iq06.rs",
            "/src/ch07_object_oriented_design/iq07.rs",
            "/src/ch07_object_oriented_design/iq08.rs",
            "/src/ch07_object_oriented_design/iq09.rs",
            "/src/ch07_object_oriented_design/iq10.rs",
            "/src/ch07_object_oriented_design/iq11.rs",
            "/src/ch07_object_oriented_design/iq12.rs",
            "/src/ch08_recursion_and_dynamic_programming/iq01.rs",
            "/src/ch08_recursion_and_dynamic_programming/iq02.rs",
            "/src/ch08_recursion_and_dynamic_programming/iq03.rs",
            "/src/ch08_recursion_and_dynamic_programming/iq04.rs",
            "/src/ch08_recursion_and_dynamic_programming/iq05.rs",
            "/src/ch08_recursion_and_dynamic_programming/iq06.rs",
            "/src/ch08_recursion_and_dynamic_programming/iq07.rs",
            "/src/ch08_recursion_and_dynamic_programming/iq08.rs",
            "/src/ch08_recursion_and_dynamic_programming/iq09.rs",
            "/src/ch08_recursion_and_dynamic_programming/iq10.rs",
            "/src/ch08_recursion_and_dynamic_programming/iq11.rs",
            "/src/ch08_recursion_and_dynamic_programming/iq12.rs",
            "/src/ch08_recursion_and_dynamic_programming/iq13.rs",
            "/src/ch08_recursion_and_dynamic_programming/iq14.rs",
            "/src/ch10_sorting_and_searching/iq01.rs",
            "/src/ch10_sorting_and_searching/iq02.rs",
            "/src/ch10_sorting_and_searching/iq03.rs",
            "/src/ch10_sorting_and_searching/iq04.rs",
            "/src/ch10_sorting_and_searching/iq05.rs",
            "/src/ch10_sorting_and_searching/iq06.rs",
            "/src/ch10_sorting_and_searching/iq07.rs",
            "/src/ch10_sorting_and_searching/iq08.rs",
            "/src/ch10_sorting_and_searching/iq09.rs",
            "/src/ch10_sorting_and_searching/iq10.rs",
            "/src/ch10_sorting_and_searching/iq11.rs",
            "/src/ch16_moderate/iq01.rs",
            "/src/ch16_moderate/iq02.rs",
            "/src/ch16_moderate/iq03.rs",
            "/src/ch16_moderate/iq04.rs",
            "/src/ch16_moderate/iq05.rs",
            "/src/ch16_moderate/iq06.rs",
            "/src/ch16_moderate/iq07.rs",
            "/src/ch16_moderate/iq08.rs",
            "/src/ch16_moderate/iq09.rs",
            "/src/ch16_moderate/iq10.rs",
            "/src/ch16_moderate/iq11.rs",
            "/src/ch16_moderate/iq12.rs",
            "/src/ch16_moderate/iq13.rs",
            "/src/ch16_moderate/iq14.rs",
            "/src/ch16_moderate/iq15.rs",
            "/src/ch16_moderate/iq16.rs",
            "/src/ch16_moderate/iq17.rs",
            "/src/ch16_moderate/iq18.rs",
            "/src/ch16_moderate/iq19.rs",
            "/src/ch16_moderate/iq20.rs",
            "/src/ch16_moderate/iq21.rs",
            "/src/ch16_moderate/iq22.rs",
            "/src/ch16_moderate/iq23.rs",
            "/src/ch16_moderate/iq24.rs",
            "/src/ch16_moderate/iq25.rs",
            "/src/ch16_moderate/iq26.rs",
            "/src/ch17_hard/iq01.rs",
            "/src/ch17_hard/iq02.rs",
            "/src/ch17_hard/iq03.rs",
            "/src/ch17_hard/iq04.rs",
            "/src/ch17_hard/iq05.rs",
            "/src/ch17_hard/iq06.rs",
            "/src/ch17_hard/iq07.rs",
            "/src/ch17_hard/iq08.rs",
            "/src/ch17_hard/iq09.rs",
            "/src/ch17_hard/iq10.rs",
            "/src/ch17_hard/iq11.rs",
            "/src/ch17_hard/iq12.rs",
            "/src/ch17_hard/iq13.rs",
            "/src/ch17_hard/iq14.rs",
            "/src/ch17_hard/iq15.rs",
            "/src/ch17_hard/iq16.rs",
            "/src/ch17_hard/iq17.rs",
            "/src/ch17_hard/iq18.rs",
            "/src/ch17_hard/iq19.rs",
            "/src/ch17_hard/iq20.rs",
            "/src/ch17_hard/iq21.rs",
            "/src/ch17_hard/iq22.rs",
            "/src/ch17_hard/iq23.rs",
            "/src/ch17_hard/iq24.rs",
            "/src/ch17_hard/iq25.rs",
            "/src/ch17_hard/iq26.rs",
        ]
    };

    for fpath in files {
        let file_path = format!("{root}{fpath}");
        let input_path = Path::new(&file_path);
        let contents = fs::read_to_string(input_path).expect("Failed to read file");

        // Parse the file as Rust code
        let syntax_tree = syn::parse_file(&contents).expect("Failed to parse file");

        let mut visitor = SliceVisitor {
            inside_item: (false, String::from("")),
            file_name: file_path.clone(),
            slices: HashMap::new(),
        };
        visitor.visit_file(&syntax_tree);

        // Construct the output file path
        let output_path = construct_output_path(input_path)?;
        let mut file = File::create(output_path)?;

        // Get the current time and format it to conform to ISO-8601
        let time_string = Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();

        writeln!(file, "// Generated on {}", time_string)?;

        for (ident, count) in visitor.slices {
            // Write the const we want to be able to use in our proc macros
            writeln!(file, "pub const {}_LEN: usize = {};", ident, count)?;
        }
    }

    Ok(())
}

fn construct_output_path(input_path: &Path) -> io::Result<PathBuf> {
    let parent_dir = input_path.parent().expect("Failed to get parent directory");
    let file_name = input_path.file_name().expect("Failed to get file name");

    // Replace the parent directory with the metadata directory
    let output_dir = parent_dir.join("metadata");
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir)?;
    }

    Ok(output_dir.join(file_name))
}
