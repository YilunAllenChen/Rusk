extern crate rusk;
use polars::prelude::*;

fn main() {

    let df = df! [
        "A"        => [1, 2, 3, 4, 5],
        "fruits"   => ["banana", "banana", "apple", "apple", "banana"],
        "B"        => [5, 4, 3, 2, 1],
        "cars"     => ["beetle", "audi", "beetle", "beetle", "beetle"],
        "optional" => [Some(28), Some(300), None, Some(2), Some(-30)],
    ];

    println!("{:?}", df);

    let res = rusk::add(1,2);
    println!("{}",res);
    println!("Hello, world!");
}
