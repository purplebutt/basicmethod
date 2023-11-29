// src/main.rs
use basicmethod::BasicMethod;

pub enum Division {
    Marketing, IT, Finance, Other
}

#[derive(BasicMethod)]
/// Sample documentation
struct Sample {
    id: i32,
    name: String,
    age: u8,
    division: Division
}

fn demo1() {
    let mut s = Sample::new(23, "Natalia".to_string(), 35, Division::IT);
    println!("{}", s.get_name());
    s.set_name("Mundo".to_string());
    println!("{}", s.get_name());
    let i = Sample::info();
    assert_eq!("Sample documentation", i.as_str());

    for (field, ftype) in Sample::fields() {
        println!("{}-> {}", field, ftype);
    }
}

#[derive(BasicMethod)]
/// A tuple struct
struct Player(String, u8);

fn demo2() {
    let mut kowi = Player("Joko".to_string(), 55);
    for field in Player::fields() {
        println!("{}", field)
    }
    kowi.set_String("Jokowido".to_string());
    kowi.set_u8(64);
    println!("{} {}", kowi.get_String(), kowi.get_u8());
}

fn main() {
    demo1();
    demo2();
}
