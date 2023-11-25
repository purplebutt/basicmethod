#![allow(unused)]

use basicmethod::BasicMethod;

pub enum Division {
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
    let mut s = Sample::new(23, "Natalia".to_string(), 33, Division::It);
    let i = Sample::info();

    println!("{}", s.get_name());
    s.set_name("Master Yi".to_string());
    println!("{}", s.get_name());

    println!("{i}");

    for (f, t) in Sample::fields() {
        println!("{}-> {}", f, t);
    }
}

