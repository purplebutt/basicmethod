# basicmethod
**Add constructor, get and set method using derive macro

* What's new in v.0.1.1
New method get_mut now available

- This crates contains derive macro that add:
*Constructor - new method 
*Getter - get method for each struct field 
*Setter - set method for each struct field 
*Info - info method to print struct documentation 
*Fields - fields method that return Vec<(&str, &str)> of fields 
**Support for struct, unit-struct and tuple-struct

## Examples - quick start
```rust
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
    kowi.set_String_0("Jokowido".to_string());
    kowi.set_u8_1(64);
    println!("{} {}", kowi.get_String_0(), kowi.get_u8_1());
}

#[derive(BasicMethod)]
struct Demo {
    val: i32
}

fn demo3() {
    let mut v = Demo::new(10);
    let mutv = v.get_val_mut();
    *mutv += 5;
    assert_eq!(15, *v.get_val());
}

fn main() {
    demo1();
    demo2();
    demo3();
}
```

