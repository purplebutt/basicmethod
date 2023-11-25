#![allow(unused)]

use basicmethod::BasicMethod;

enum Division {
    Marketing,
    It,
    Finance,
    Production,
    HR,
    Other
}

#[derive(BasicMethod)]
/// Sample documentation
struct Sample {
    id: i32,
    name: String,
    age: u8,
    division: Division
}


fn main() {
    let s = Sample::new(23, "Natalia".to_string(), 33, Division::It);
    let i = Sample::info();

    println!("{i}");
    for (f, t) in Sample::fields() {
        println!("{}-> {}", f, t);
    }
}
