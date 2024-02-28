pub fn fizzbuzz0(limit: i32) -> String {
    let mut res = String::new();
    for idx in 1..limit + 1 {
        if idx % 3 == 0 && idx % 5 == 0 {
            res.push_str("FizzBuzz");
        } else if idx % 3 == 0 {
            res.push_str("Fizz");
        } else if idx % 5 == 0 {
            res.push_str("Buzz");
        } else {
            res.push_str(&idx.to_string());
        }
    }
    res
}

pub fn fizzbuzz1(limit: i32) -> String {
    let mut res = String::new();
    for idx in 1..limit + 1 {
        match idx {
            _ if idx % 15 == 0 => res.push_str("FizzBuzz"),
            _ if idx % 3 == 0 => res.push_str("Fizz"),
            _ if idx % 5 == 0 => res.push_str("Buzz"),
            _ => res.push_str(idx.to_string().as_str()),
        }
    }
    res
}

pub fn fizzbuzz2(limit: i32) -> String {
    let mut res = String::with_capacity(limit as usize * 8);

    for idx in 1..limit + 1 {
        match (idx % 3, idx % 5) {
            (0, 0) => res.push_str("FizzBuzz"),
            (0, _) => res.push_str("Fizz"),
            (_, 0) => res.push_str("Buzz"),
            _ => {
                use std::fmt::Write;
                let _ = write!(res, "{}", idx);
            }
        }
    }
    res
}

pub const INPUT_EXPECTED: &[(i32, &str)] = &[
    (1, "1"),
    (2, "12"),
    (3, "12Fizz"),
    (4, "12Fizz4"),
    (5, "12Fizz4Buzz"),
    (6, "12Fizz4BuzzFizz"),
    (10, "12Fizz4BuzzFizz78FizzBuzz"),
    (15, "12Fizz4BuzzFizz78FizzBuzz11Fizz1314FizzBuzz"),
    (20, "12Fizz4BuzzFizz78FizzBuzz11Fizz1314FizzBuzz1617Fizz19Buzz"),
    (30, "12Fizz4BuzzFizz78FizzBuzz11Fizz1314FizzBuzz1617Fizz19BuzzFizz2223FizzBuzz26Fizz2829FizzBuzz"),
    (50, "12Fizz4BuzzFizz78FizzBuzz11Fizz1314FizzBuzz1617Fizz19BuzzFizz2223FizzBuzz26Fizz2829FizzBuzz3132Fizz34BuzzFizz3738FizzBuzz41Fizz4344FizzBuzz4647Fizz49Buzz"),
    (100, "12Fizz4BuzzFizz78FizzBuzz11Fizz1314FizzBuzz1617Fizz19BuzzFizz2223FizzBuzz26Fizz2829FizzBuzz3132Fizz34BuzzFizz3738FizzBuzz41Fizz4344FizzBuzz4647Fizz49BuzzFizz5253FizzBuzz56Fizz5859FizzBuzz6162Fizz64BuzzFizz6768FizzBuzz71Fizz7374FizzBuzz7677Fizz79BuzzFizz8283FizzBuzz86Fizz8889FizzBuzz9192Fizz94BuzzFizz9798FizzBuzz"),
];

#[cfg(test)]
mod tests {
    use super::*;
    use gen_tests::generate_tests;

    fn str_eq(a: &String, b: &str) -> bool {
        a.as_str() == b
    }

    fn strlen_eq(a: &String, b: &str) -> bool {
        a.as_str().len() == b.len()
    }

    generate_tests!(fizzbuzz0, 12, str_eq);
    generate_tests!(fizzbuzz1, 12, str_eq);
    generate_tests!(fizzbuzz2, 12, str_eq);

    generate_tests!(fizzbuzz0, 12, strlen_eq);
    generate_tests!(fizzbuzz1, 12, strlen_eq);
    generate_tests!(fizzbuzz2, 12, strlen_eq);
}
