#![feature(concat_idents)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Ident, Token};

struct GenerateTestsInput {
    // Function name identifier
    ident: Ident,

    // Token to separate it from the variable that holds the length of the
    // test input list
    _comma: Token![,],

    // Lenght of the test input list
    usize_val: syn::LitInt,
}

impl Parse for GenerateTestsInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            _comma: input.parse()?,
            usize_val: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn generate_tests(input: TokenStream) -> TokenStream {
    let its = parse_macro_input!(input as GenerateTestsInput);
    let func = its.ident.clone();
    let func_name = its.ident.to_string();

    // Parse the LitInt to usize
    //let max_len = concat_idents!(const TESTS_LEN);
    let max_len = its.usize_val.base10_parse::<usize>().unwrap();

    // Assuming a fixed length for simplicity; in practice, you'd pass this in or compute it.
    let tests = (0..max_len).map(|index| {
        let test_name = format!("{}_{}", func_name, index);
        let test_ident = Ident::new(&test_name, func.span());

        quote! {
            #[test]
            fn #test_ident() {
                let (test_input, expected) = INPUT_EXPECTED[#index];
                let result = #func(test_input);
                assert!(result == expected, "Failed on input: {:?}", test_input);
            }
        }
    });

    TokenStream::from(quote! {
        #(#tests)*
    })
}

// Create a function-like procedural macro that takes a usize as an argument
#[proc_macro]
pub fn int2lit(input: TokenStream) -> TokenStream {
    // Parse the input token stream to extract the argument
    let arg = syn::parse_macro_input!(input as syn::LitInt);

    // Convert the argument to a usize
    let arg_usize = arg.base10_parse::<i32>().unwrap();

    // Return a token stream with the argument value
    TokenStream::from(quote! {#(#arg_usize)})
}
