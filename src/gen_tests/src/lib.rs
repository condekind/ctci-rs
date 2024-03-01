extern crate proc_macro;

mod gen_tests_mod {

    use syn;
    use syn::parse::{Parse, ParseStream, Result};
    use syn::{Ident, Token};

    pub struct GenerateTestsInput {
        // Function name identifier
        pub test_func: Ident,

        // Token to separate it from the variable that holds the length of the
        // test input list
        pub _comma0: Token![,],

        // Lenght of the test input list
        pub usize_val: syn::LitInt,

        // Token to separate the variable that holds the length of the test input
        // list from the function that will perform the equality check (or similar)
        pub _comma1: Token![,],

        // Identifier for the custom assertion function
        pub assert_func: Ident,
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
}

use gen_tests_mod::GenerateTestsInput;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn generate_tests(input: TokenStream) -> TokenStream {
    let its = parse_macro_input!(input as GenerateTestsInput);
    let func_ident = its.test_func.clone();
    let func_name = its.test_func.to_string();
    let assert_func = its.assert_func.clone();
    let assert_func_name = its.assert_func.to_string();

    // Parse the LitInt to usize
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

mod gen_tests_mod2 {
    use syn::{
        parse::{Parse, ParseStream},
        Ident, LitInt, Result, Token,
    };

    pub struct GenTestsInput {
        // Function name identifier
        pub ident: Ident,

        // Token to separate it from the variable that holds the length of the
        // test input list
        pub _comma: Token![,],

        // Length of the test input list
        pub usize_val: LitInt,

        // Optional: Token to separate the length of the test input list from the assertion function
        pub _comma2: Option<Token![,]>,

        // Optional: Identifier for the custom assertion function
        pub assert_func: Option<Ident>,
    }

    impl Parse for GenTestsInput {
        fn parse(input: ParseStream) -> Result<Self> {
            let ident: Ident = input.parse()?;
            let _comma: Token![,] = input.parse()?;
            let usize_val: LitInt = input.parse()?;

            let lookahead = input.lookahead1();
            let (_comma2, assert_func) = if lookahead.peek(Token![,]) {
                let _comma2: Token![,] = input.parse()?;
                let assert_func: Ident = input.parse()?;
                (Some(_comma2), Some(assert_func))
            } else {
                (None, None)
            };

            Ok(Self {
                ident,
                _comma,
                usize_val,
                _comma2,
                assert_func,
            })
        }
    }
}

use extra::extra::*;
use gen_tests_mod2::GenTestsInput;

#[proc_macro]
pub fn generate_tests2(input: TokenStream) -> TokenStream {
    let its = parse_macro_input!(input as GenTestsInput);
    let func = its.ident.clone();
    let func_name = its.ident.to_string();
    let max_len = its.usize_val.base10_parse::<usize>().unwrap();

    let tests = (0..max_len).map(|index| match its.assert_func.as_ref() {
        Some(assert_func) => {
            let test_name = format!("{}_{}_{:03}", func_name, assert_func.to_string(), index);
            let test_ident = Ident::new(&test_name, func.span());
            quote! {
                #[test]
                fn #test_ident() {
                    let (test_input, expected) = INPUT_EXPECTED[#index];
                    let result = #func(test_input);
                    assert!(#assert_func(&result, expected), "Failed on input: {:?}", test_input);
                }
            }
        }
        None => {
            let test_name = format!("{}_assert_{:03}", func_name, index);
            let test_ident = Ident::new(&test_name, func.span());
            quote! {
                #[test]
                fn #test_ident() {
                    let (test_input, expected) = INPUT_EXPECTED[#index];
                    let result = #func(test_input);
                    assert!(result == expected, "Failed on input: {:?}", test_input);
                }
            }
        }
    });

    TokenStream::from(quote! {
        #(#tests)*
    })
}
