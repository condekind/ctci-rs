fn main() {
    let _ = match parse_tests::main() {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {e:?}")
        }
    };
}
