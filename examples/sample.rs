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

#[derive(BasicMethod)]
/// A tuple struct contains i32
struct Unit(i32, String);

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

    let mut u = Unit(10, "hello".into());

    for tf in Unit::fields() {
        println!("{}", tf)
    }

    u.set_i32_0(20);
    u.set_String_1("Love you".to_string());

    println!("{} {}", u.get_i32_0(), u.get_String_1());
}

