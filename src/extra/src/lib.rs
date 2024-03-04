pub mod extra {
    use std::marker::PhantomData;

    pub trait Solution<T, U> {
        fn solve(&self, args: T) -> U;
        fn check_output(&self, expected: &U, result: U) -> bool;
    }

    // This struct implements Solution. The user is supposed to construct it with
    // one of the factory functions: `with_custom_check`, `with_default_check` or
    // `new` (which is just an alias for `with_default_check`). Its fields are not
    // public, so it can't be instantiated directly
    pub struct Solver<T, U, F, G>
    where
        F: Fn(T) -> U + 'static,
        G: Fn(&U, U) -> bool + 'static,
    {
        solve_fn: Box<F>,
        check_output_fn: Box<G>,
        _marker: PhantomData<(T, U)>,
    }

    // Provides a way to construct a Solver, plus setters for the solve() and
    // check_output() functions
    impl<T, U, F> Solver<T, U, F, fn(&U, U) -> bool>
    where
        F: Fn(T) -> U + 'static,
    {
        // Main ctor (factory function)
        pub fn with_custom_check(solve_fn: F, check_output_fn: fn(&U, U) -> bool) -> Self {
            Self {
                solve_fn: Box::new(solve_fn),
                check_output_fn: Box::new(check_output_fn),
                _marker: PhantomData,
            }
        }

        // Setter for the solve() function
        pub fn set_solve_fn(&mut self, f: F) {
            self.solve_fn = Box::new(f);
        }

        // Setter for the check_output() function
        pub fn set_check_output_fn(&mut self, f: fn(&U, U) -> bool) {
            self.check_output_fn = Box::new(f);
        }
    }

    // This alternative impl includes a trait bound on U that is only used for
    // when the user wishes to use the default check function.
    //
    // The setter functions of the first impl. block for Solver (above) are
    // also available for Solvers constructed `with_default_check`
    impl<T, U, F> Solver<T, U, F, fn(&U, U) -> bool>
    where
        F: Fn(T) -> U + 'static,
        U: PartialEq,
    {
        // Alternative ctor (factory) with a default check_output_fn that
        // requires `U: PartialEq`
        pub fn with_default_check(solve_fn: F) -> Self {
            Self {
                solve_fn: Box::new(solve_fn),

                // Default check function
                check_output_fn: Box::new(|a, b| *a == b),

                // This just exists to tell the compiler this struct logically
                // contains data of types T and U, but doesn't physically store
                // them. It marks fields of unused types, in a context
                _marker: PhantomData,
            }
        }

        // This `new` function acts as an alias to `with_default_check`
        pub fn new(solve_fn: F) -> Self {
            Self::with_default_check(solve_fn)
        }
    }

    // Here's where Solver implements the trait Solution:
    // * solve() just calls the function with the args passed to it.
    // Suggestion: unless you only have ONE trivially simple arg, wrap the args in
    // a tuple struct and code your solution function to take it and destructure it
    // at the beginning. This will allow you to leverage the generate_tests! macro
    impl<T, U, F, G> Solution<T, U> for Solver<T, U, F, G>
    where
        F: Fn(T) -> U + 'static,
        G: Fn(&U, U) -> bool + 'static,
    {
        fn solve(&self, args: T) -> U {
            (self.solve_fn)(args)
        }

        fn check_output(&self, expected: &U, result: U) -> bool {
            (self.check_output_fn)(expected, result)
        }
    }
}
