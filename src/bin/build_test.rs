extern crate chrono;

use chrono::prelude::*;
use std::any::Any;
use std::fmt::Debug;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use syn::visit_mut::VisitMut;
use syn::{self, visit::Visit, Expr, ExprArray, ExprAssign, ExprConst, ExprField, ExprTuple, Fields, ItemConst, ItemMod, Lit, TypeArray, Variant, Attribute, Visibility, Ident, Generics, Type, Token, ItemStatic, StaticMutability};
use syn::token::{Colon, Const, Semi};

struct Visitor {
    count: usize,
    filename: &'static str,
}

impl<'ast> Visit<'ast> for Visitor {
    fn visit_expr(&mut self, expr: &'ast Expr) {
        let e = expr.clone();
        let _ = match e {
            Expr::Array(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Assign(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Async(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Await(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Binary(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Block(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Break(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Call(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Cast(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Closure(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Const(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Continue(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Field(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::ForLoop(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Group(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::If(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Index(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Infer(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Let(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Lit(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Loop(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Macro(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Match(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::MethodCall(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Paren(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Path(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Range(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Reference(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Repeat(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Return(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Struct(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Try(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::TryBlock(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Tuple(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Unary(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Unsafe(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Verbatim(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::While(expr) => {
                println!("{:?}\n", expr)
            }
            Expr::Yield(expr) => {
                println!("{:?}\n", expr)
            }
            _ => (),
        };

        //println!("{:?}", expr);
    }








    fn visit_item_static(&mut self, item: &'ast ItemStatic) {

        let _ = match item {
            ItemStatic {
                attrs: curr_attrs,
                vis: curr_vis,
                static_token: curr_static_token,
                mutability: curr_mutability,
                ident: curr_ident,
                colon_token: curr_colon_token,
                ty: curr_ty,
                eq_token: curr_eq_token,
                expr: curr_expr,
                semi_token: curr_semi_token,
            } => {
                if curr_ident == "INPUT_EXPECTED" {
                    println!("found list!");


                } else if curr_ident == "INPUT_LEN" {
                    println!("found len!");
                }
            }
        };
    }

    fn visit_item_const(&mut self, item: &'ast ItemConst) {
        let _ = match item {
            ItemConst {
                attrs: curr_attrs,
                vis: curr_vis,
                const_token: curr_const_token,
                ident: curr_ident,
                generics: curr_generics,
                colon_token: curr_colon_token,
                ty: curr_ty,
                eq_token: curr_eq_token,
                expr: curr_expr,
                semi_token: curr_semi_token,
            } => {
                if curr_ident == "INPUT_EXPECTED" {
                    println!("found!");
                }
            }
        };









        if item.ident == "INPUT_EXPECTED" {
            let f = *item.expr.clone();
            if let syn::Expr::Array(array) = &*item.expr {
                self.count += array.elems.len();
            }
        }
    }

    fn visit_type_array(&mut self, i: &'ast TypeArray) {
        todo!()
    }

    /*
    fn visit_item_const(&mut self, item: &'ast ItemConst) {
        let i = item.clone();
        let _ = match i {
            ItemConst {
                attrs,
                vis,
                const_token,
                ident,
                generics,
                colon_token,
                ty,
                eq_token,
                expr,
                semi_token,
            } => {
                println!("attrs={:?}\n", attrs);
                println!("vis={:?}\n", vis);
                println!("const_token={:?}\n", const_token);
                println!("ident={:?}\n", ident);
                println!("generics={:?}\n", generics);
                println!("colon_token={:?}\n", colon_token);
                println!("ty={:?}\n", ty);
                println!("eq_token={:?}\n", eq_token);
                println!("expr={:?}\n", expr);
                println!("semi_token={:?}\n", semi_token);
            }
        };
    }
    */

    fn visit_expr_array(&mut self, expr_array: &'ast ExprArray) {
        let f = expr_array.clone();
    }

    fn visit_expr_assign(&mut self, expr_assign: &'ast ExprAssign) {
        let expr = expr_assign.clone();
    }

    fn visit_expr_const(&mut self, expr_const: &'ast ExprConst) {
        let i = expr_const.clone();
        /*
        if expr_const.ident == "INPUT_EXPECTED" {
            //let f = *item.expr.clone();
            if let syn::Expr::Array(array) = &*expr_const.expr {
                self.count += array.elems.len();
            }
        }
        */
    }
}

struct StaticSliceVisitor {
    // A flag to indicate whether the current static item is INPUT_EXPECTED
    //curr_slice: Vec<(&'static str, usize)>,
    is_input_expected: bool,
}

impl<'ast> Visit<'ast> for StaticSliceVisitor {
    fn visit_item_static(&mut self, item: &'ast ItemStatic) {
        // Check if the static item's identifier is "INPUT_EXPECTED"
        println!("Found ItemStatic named {:?}\n", item.ident);
        if item.ident == "INPUT_EXPECTED" {
            //println!("Found INPUT_EXPECTED: {:?}\n", item);
            self.is_input_expected = true;
            let debug_print = true;
            let f = match &*item.expr {
                Expr::Reference(ex) => {
                    let e = match &*ex.expr {
                        Expr::Array(expr) => {
                            println!("-------------------------- ARRAY!!!! --------------------------");
                            //println!("{:?}\n", expr);
                            self.visit_expr_array(expr);
                        }
                        /*
                        Expr::Array(expr) => {
                            println!("The Expr is Array");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        */
                        Expr::Assign(expr) => {
                            println!("The Expr is Assign");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Async(expr) => {
                            println!("The Expr is Async");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Await(expr) => {
                            println!("The Expr is Await");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Binary(expr) => {
                            println!("The Expr is Binary");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Block(expr) => {
                            println!("The Expr is Block");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Break(expr) => {
                            println!("The Expr is Break");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Call(expr) => {
                            println!("The Expr is Call");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Cast(expr) => {
                            println!("The Expr is Cast");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Closure(expr) => {
                            println!("The Expr is Closure");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Const(expr) => {
                            println!("The Expr is Const");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Continue(expr) => {
                            println!("The Expr is Continue");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Field(expr) => {
                            println!("The Expr is Field");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::ForLoop(expr) => {
                            println!("The Expr is ForLoop");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Group(expr) => {
                            println!("The Expr is Group");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::If(expr) => {
                            println!("The Expr is If");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Index(expr) => {
                            println!("The Expr is Index");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Infer(expr) => {
                            println!("The Expr is Infer");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Let(expr) => {
                            println!("The Expr is Let");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Lit(expr) => {
                            println!("The Expr is Lit");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Loop(expr) => {
                            println!("The Expr is Loop");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Macro(expr) => {
                            println!("The Expr is Macro");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Match(expr) => {
                            println!("The Expr is Match");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::MethodCall(expr) => {
                            println!("The Expr is MethodCall");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Paren(expr) => {
                            println!("The Expr is Paren");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Path(expr) => {
                            println!("The Expr is Path");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Range(expr) => {
                            println!("The Expr is Range");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Reference(expr) => {
                            println!("The Expr is Reference");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Repeat(expr) => {
                            println!("The Expr is Repeat");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Return(expr) => {
                            println!("The Expr is Return");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Struct(expr) => {
                            println!("The Expr is Struct");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Try(expr) => {
                            println!("The Expr is Try");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::TryBlock(expr) => {
                            println!("The Expr is TryBlock");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Tuple(expr) => {
                            println!("The Expr is Tuple");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Unary(expr) => {
                            println!("The Expr is Unary");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Unsafe(expr) => {
                            println!("The Expr is Unsafe");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Verbatim(expr) => {
                            println!("The Expr is Verbatim");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::While(expr) => {
                            println!("The Expr is While");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        Expr::Yield(expr) => {
                            println!("The Expr is Yield");
                            if debug_print { println!("{:?}\n", expr); }
                        }
                        _ => {
                            let f = &*item.expr;
                            println!("Found something... {:?}\n", f);
                            ()
                        }
                    };
                }
                _ => ()
            };
            self.is_input_expected = false;
        }

    }

    fn visit_expr_array(&mut self, expr: &'ast ExprArray) {
        // Check if we're within the INPUT_EXPECTED static item
        if self.is_input_expected {
            println!("yo!");
            println!("Length of INPUT_EXPECTED: {}", expr.elems.len());
        }

        // Continue visiting other expressions
        //syn::visit::visit_expr(self, expr);
    }
}

/*
impl<'ast> Visit<'ast> for StaticSliceVisitor {

    /*
    fn visit_item_static(&mut self, i: &'ast ItemStatic) {

        let mut foo = |this: &mut Self, ident: Ident, expr_array: &syn::ExprArray| {
            println!("Length of the {} slice: {}", ident, expr_array.elems.len());
        };

        let mut ff: Box<Expr>;
        if i.ident == "INPUT_EXPECTED" {
            // Directly use a reference to the expression without cloning

            let _ = match i.clone() {
                ItemStatic {
                    attrs: curr_attrs,
                    vis: curr_vis,
                    static_token: curr_static_token,
                    mutability: curr_mutability,
                    ident: curr_ident,
                    colon_token: curr_colon_token,
                    ty: curr_ty,
                    eq_token: curr_eq_token,
                    expr: curr_expr,
                    semi_token: curr_semi_token,
                } => {
                    if curr_ident != "INPUT_EXPECTED" {
                        return
                    }
                    ff = curr_expr.clone();
                    println!("Found static {:?} of {:?}\n", curr_ident, curr_ty);
                    if let syn::Expr::Array(expr_array) = &*curr_expr {
                        foo(self, curr_ident, expr_array);
                    }
                }
            };

            //if let syn::Expr::Array(expr_array) = &*i.expr {
            //    foo(self, expr_array);
            //}
        }
    }
    */

    //fn visit_expr_array(&mut self, i: &'ast ExprArray) {
    //    println!("Length of the GLOBAL_DATA slice: {}", i.elems.len());
    //    // Continue walking down the AST if needed
    //    syn::visit::visit_expr_array(self, i);
    //}

}
*/

fn main() -> io::Result<()> {
    let files = vec![
        "/home/condekind/repos/ctci-rs/src/ch01_arrays_and_strings/iq01.rs",
        //"/home/condekind/repos/ctci-rs/src/ch01_arrays_and_strings/iq02.rs",
        //"/home/condekind/repos/ctci-rs/src/ch01_arrays_and_strings/iq03.rs",
        //"/home/condekind/repos/ctci-rs/src/ch01_arrays_and_strings/iq04.rs",
        //"/home/condekind/repos/ctci-rs/src/ch01_arrays_and_strings/iq05.rs",
        //"/home/condekind/repos/ctci-rs/src/ch01_arrays_and_strings/iq06.rs",
        //"/home/condekind/repos/ctci-rs/src/ch01_arrays_and_strings/iq07.rs",
        //"/home/condekind/repos/ctci-rs/src/ch01_arrays_and_strings/iq08.rs",
        //"/home/condekind/repos/ctci-rs/src/ch01_arrays_and_strings/iq09.rs",
        //"/home/condekind/repos/ctci-rs/src/ch02_linked_lists/iq01.rs",
        //"/home/condekind/repos/ctci-rs/src/ch02_linked_lists/iq02.rs",
        //"/home/condekind/repos/ctci-rs/src/ch02_linked_lists/iq03.rs",
        //"/home/condekind/repos/ctci-rs/src/ch02_linked_lists/iq04.rs",
        //"/home/condekind/repos/ctci-rs/src/ch02_linked_lists/iq05.rs",
        //"/home/condekind/repos/ctci-rs/src/ch02_linked_lists/iq06.rs",
        //"/home/condekind/repos/ctci-rs/src/ch02_linked_lists/iq07.rs",
        //"/home/condekind/repos/ctci-rs/src/ch02_linked_lists/iq08.rs",
        //"/home/condekind/repos/ctci-rs/src/ch03_stacks_and_queues/iq01.rs",
        //"/home/condekind/repos/ctci-rs/src/ch03_stacks_and_queues/iq02.rs",
        //"/home/condekind/repos/ctci-rs/src/ch03_stacks_and_queues/iq03.rs",
        //"/home/condekind/repos/ctci-rs/src/ch03_stacks_and_queues/iq04.rs",
        //"/home/condekind/repos/ctci-rs/src/ch03_stacks_and_queues/iq05.rs",
        //"/home/condekind/repos/ctci-rs/src/ch03_stacks_and_queues/iq06.rs",
        //"/home/condekind/repos/ctci-rs/src/ch04_trees_and_graphs/iq01.rs",
        //"/home/condekind/repos/ctci-rs/src/ch04_trees_and_graphs/iq02.rs",
        //"/home/condekind/repos/ctci-rs/src/ch04_trees_and_graphs/iq03.rs",
        //"/home/condekind/repos/ctci-rs/src/ch04_trees_and_graphs/iq04.rs",
        //"/home/condekind/repos/ctci-rs/src/ch04_trees_and_graphs/iq05.rs",
        //"/home/condekind/repos/ctci-rs/src/ch04_trees_and_graphs/iq06.rs",
        //"/home/condekind/repos/ctci-rs/src/ch04_trees_and_graphs/iq07.rs",
        //"/home/condekind/repos/ctci-rs/src/ch04_trees_and_graphs/iq08.rs",
        //"/home/condekind/repos/ctci-rs/src/ch04_trees_and_graphs/iq09.rs",
        //"/home/condekind/repos/ctci-rs/src/ch04_trees_and_graphs/iq10.rs",
        //"/home/condekind/repos/ctci-rs/src/ch04_trees_and_graphs/iq11.rs",
        //"/home/condekind/repos/ctci-rs/src/ch04_trees_and_graphs/iq12.rs",
        //"/home/condekind/repos/ctci-rs/src/ch05_bit_manipulation/iq01.rs",
        //"/home/condekind/repos/ctci-rs/src/ch05_bit_manipulation/iq02.rs",
        //"/home/condekind/repos/ctci-rs/src/ch05_bit_manipulation/iq03.rs",
        //"/home/condekind/repos/ctci-rs/src/ch05_bit_manipulation/iq04.rs",
        //"/home/condekind/repos/ctci-rs/src/ch05_bit_manipulation/iq05.rs",
        //"/home/condekind/repos/ctci-rs/src/ch05_bit_manipulation/iq06.rs",
        //"/home/condekind/repos/ctci-rs/src/ch05_bit_manipulation/iq07.rs",
        //"/home/condekind/repos/ctci-rs/src/ch05_bit_manipulation/iq08.rs",
        //"/home/condekind/repos/ctci-rs/src/ch06_math_and_logic_puzzles/iq01.rs",
        //"/home/condekind/repos/ctci-rs/src/ch06_math_and_logic_puzzles/iq02.rs",
        //"/home/condekind/repos/ctci-rs/src/ch06_math_and_logic_puzzles/iq03.rs",
        //"/home/condekind/repos/ctci-rs/src/ch06_math_and_logic_puzzles/iq04.rs",
        //"/home/condekind/repos/ctci-rs/src/ch06_math_and_logic_puzzles/iq05.rs",
        //"/home/condekind/repos/ctci-rs/src/ch06_math_and_logic_puzzles/iq06.rs",
        //"/home/condekind/repos/ctci-rs/src/ch06_math_and_logic_puzzles/iq07.rs",
        //"/home/condekind/repos/ctci-rs/src/ch06_math_and_logic_puzzles/iq08.rs",
        //"/home/condekind/repos/ctci-rs/src/ch06_math_and_logic_puzzles/iq09.rs",
        //"/home/condekind/repos/ctci-rs/src/ch06_math_and_logic_puzzles/iq10.rs",
        //"/home/condekind/repos/ctci-rs/src/ch07_object_oriented_design/iq01.rs",
        //"/home/condekind/repos/ctci-rs/src/ch07_object_oriented_design/iq02.rs",
        //"/home/condekind/repos/ctci-rs/src/ch07_object_oriented_design/iq03.rs",
        //"/home/condekind/repos/ctci-rs/src/ch07_object_oriented_design/iq04.rs",
        //"/home/condekind/repos/ctci-rs/src/ch07_object_oriented_design/iq05.rs",
        //"/home/condekind/repos/ctci-rs/src/ch07_object_oriented_design/iq06.rs",
        //"/home/condekind/repos/ctci-rs/src/ch07_object_oriented_design/iq07.rs",
        //"/home/condekind/repos/ctci-rs/src/ch07_object_oriented_design/iq08.rs",
        //"/home/condekind/repos/ctci-rs/src/ch07_object_oriented_design/iq09.rs",
        //"/home/condekind/repos/ctci-rs/src/ch07_object_oriented_design/iq10.rs",
        //"/home/condekind/repos/ctci-rs/src/ch07_object_oriented_design/iq11.rs",
        //"/home/condekind/repos/ctci-rs/src/ch07_object_oriented_design/iq12.rs",
        //"/home/condekind/repos/ctci-rs/src/ch08_recursion_and_dynamic_programming/iq01.rs",
        //"/home/condekind/repos/ctci-rs/src/ch08_recursion_and_dynamic_programming/iq02.rs",
        //"/home/condekind/repos/ctci-rs/src/ch08_recursion_and_dynamic_programming/iq03.rs",
        //"/home/condekind/repos/ctci-rs/src/ch08_recursion_and_dynamic_programming/iq04.rs",
        //"/home/condekind/repos/ctci-rs/src/ch08_recursion_and_dynamic_programming/iq05.rs",
        //"/home/condekind/repos/ctci-rs/src/ch08_recursion_and_dynamic_programming/iq06.rs",
        //"/home/condekind/repos/ctci-rs/src/ch08_recursion_and_dynamic_programming/iq07.rs",
        //"/home/condekind/repos/ctci-rs/src/ch08_recursion_and_dynamic_programming/iq08.rs",
        //"/home/condekind/repos/ctci-rs/src/ch08_recursion_and_dynamic_programming/iq09.rs",
        //"/home/condekind/repos/ctci-rs/src/ch08_recursion_and_dynamic_programming/iq10.rs",
        //"/home/condekind/repos/ctci-rs/src/ch08_recursion_and_dynamic_programming/iq11.rs",
        //"/home/condekind/repos/ctci-rs/src/ch08_recursion_and_dynamic_programming/iq12.rs",
        //"/home/condekind/repos/ctci-rs/src/ch08_recursion_and_dynamic_programming/iq13.rs",
        //"/home/condekind/repos/ctci-rs/src/ch08_recursion_and_dynamic_programming/iq14.rs",
        //"/home/condekind/repos/ctci-rs/src/ch10_sorting_and_searching/iq01.rs",
        //"/home/condekind/repos/ctci-rs/src/ch10_sorting_and_searching/iq02.rs",
        //"/home/condekind/repos/ctci-rs/src/ch10_sorting_and_searching/iq03.rs",
        //"/home/condekind/repos/ctci-rs/src/ch10_sorting_and_searching/iq04.rs",
        //"/home/condekind/repos/ctci-rs/src/ch10_sorting_and_searching/iq05.rs",
        //"/home/condekind/repos/ctci-rs/src/ch10_sorting_and_searching/iq06.rs",
        //"/home/condekind/repos/ctci-rs/src/ch10_sorting_and_searching/iq07.rs",
        //"/home/condekind/repos/ctci-rs/src/ch10_sorting_and_searching/iq08.rs",
        //"/home/condekind/repos/ctci-rs/src/ch10_sorting_and_searching/iq09.rs",
        //"/home/condekind/repos/ctci-rs/src/ch10_sorting_and_searching/iq10.rs",
        //"/home/condekind/repos/ctci-rs/src/ch10_sorting_and_searching/iq11.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq01.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq02.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq03.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq04.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq05.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq06.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq07.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq08.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq09.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq10.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq11.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq12.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq13.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq14.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq15.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq16.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq17.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq18.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq19.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq20.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq21.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq22.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq23.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq24.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq25.rs",
        //"/home/condekind/repos/ctci-rs/src/ch16_moderate/iq26.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq01.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq02.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq03.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq04.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq05.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq06.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq07.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq08.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq09.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq10.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq11.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq12.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq13.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq14.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq15.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq16.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq17.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq18.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq19.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq20.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq21.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq22.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq23.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq24.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq25.rs",
        //"/home/condekind/repos/ctci-rs/src/ch17_hard/iq26.rs",
    ];

    for file_path in files {
        let input_path = Path::new(&file_path);
        let contents = fs::read_to_string(input_path).expect("Failed to read file");

        // Parse the file as Rust code
        let syntax_tree = syn::parse_file(&contents).expect("Failed to parse file");

        let mut visitor = StaticSliceVisitor{ is_input_expected: false };
        visitor.visit_file(&syntax_tree);

        /*

        // Create a visitor to find the INPUT_EXPECTED array length
        let mut visitor = Visitor {
            count: 0usize,
            filename: file_path,
        };
        visitor.visit_file(&syntax_tree);

        // Determine the count, defaulting to 0 if not found
        let count = visitor.count;

        // Construct the output file path
        let output_path = construct_output_path(input_path, count)?;
        let mut file = File::create(output_path)?;

        // Get the current time
        let now = Utc::now();
        // Format the time as a string in RFC-3339 format
        let time_string = now.format("%Y-%m-%dT%H:%M:%S").to_string();
        writeln!(file, "// Generated on {}", time_string)?;

        // Write the const we want to be able to use in our proc macros
        writeln!(file, "pub const LIST_LEN: usize = {};", count)?;

        */
    }

    Ok(())
}

fn construct_output_path(input_path: &Path, count: usize) -> io::Result<PathBuf> {
    let parent_dir = input_path.parent().expect("Failed to get parent directory");
    let file_name = input_path.file_name().expect("Failed to get file name");

    // Replace the parent directory with the metadata directory
    let output_dir = parent_dir.join("metadata");
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir)?;
    }

    Ok(output_dir.join(file_name))
}
