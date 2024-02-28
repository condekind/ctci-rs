#![feature(concat_idents)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Ident, Token};

struct GenerateTestsInput {
    // Function name identifier
    test_func: Ident,

    // Token to separate it from the variable that holds the length of the
    // test input list
    _comma0: Token![,],

    // Lenght of the test input list
    usize_val: syn::LitInt,

    // Token to separate the variable that holds the length of the test input
    // list from the function that will perform the equality check (or similar)
    _comma1: Token![,],

    // Identifier for the custom assertion function
    assert_func: Ident,
}

impl Parse for GenerateTestsInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            test_func: input.parse()?,
            _comma0: input.parse()?,
            usize_val: input.parse()?,
            _comma1: input.parse()?,
            assert_func: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn generate_tests(input: TokenStream) -> TokenStream {
    let its = parse_macro_input!(input as GenerateTestsInput);
    let func_ident = its.test_func.clone();
    let func_name = its.test_func.to_string();
    let assert_func = its.assert_func.clone();
    let assert_func_name = its.assert_func.to_string();

    // Parse the LitInt to usize
    //let max_len = concat_idents!(const TESTS_LEN);
    let max_len = its.usize_val.base10_parse::<usize>().unwrap();

    // Assuming a fixed length for simplicity; in practice, you'd pass this in or compute it.
    let tests = (0..max_len).map(|index| {
        let test_name = format!("{}_{}_{:03}", func_name, assert_func_name, index);
        let test_ident = Ident::new(&test_name, func_ident.span());

        quote! {
            #[test]
            fn #test_ident() {
                let (test_input, expected) = INPUT_EXPECTED[#index];
                let result = #func_ident(test_input);
                assert!(#assert_func(&result, expected), "Failed on input: {:?}", test_input);
            }
        }
    });

    TokenStream::from(quote! {
        #(#tests)*
    })
}
