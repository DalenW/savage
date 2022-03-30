use num::{ToPrimitive};
use savage_macros::function;

use crate::{expression::Integer, functions::NonNegativeInteger};
use crate::expression::Expression;

#[function(
name = "riemann_sum",
description = "sums the first n terms of the Riemann zeta function",
examples = r#"[
        ("riemann_sum(0, 3)", "12"),
        ("factorial(1)", "1"),
        ("factorial(4)", "24"),
        ("factorial(10)", "3628800"),
    ]"#,
categories = r#"[
        "calculus",
    ]"#
)]
fn riemann_sum(start_num: Integer, times: NonNegativeInteger) -> Expression {
    let f = "x + 3";
    let var = "x";
    //let times: NonNegativeInteger = t;

    // create sum_array with length of times
    let mut sum_array: Vec<Expression> = vec!["0".parse::<Expression>().unwrap(); times.to_usize().unwrap()];

    for i in 0..sum_array.len() {
        let start_number: Integer = start_num.clone();
        let num: Integer = start_number + Integer::from(i);

        // replace all instances of var in f with num
        let mut new_f = f.to_string();
        new_f = new_f.replace(var, &num.to_string());

        // evaluate new_f
        let result = new_f.parse::<Expression>().unwrap();

        // add result to sum_array
        sum_array[i] = result;
    }

    // sum the array
    let mut sum: Expression = "0".parse::<Expression>().unwrap();
    for i in sum_array {
        sum += i;
    }

    sum
}