// src/main.rs
use basicmethod::BasicMethod;

pub enum Division {
    Marketing, IT, Finance, Other
}

#[derive(BasicMethod)]
#[allow(unused)]
/// Sample documentation
struct Sample {
    #[only="get"] id: i32,          // support only get method
    #[exclude] name: String,        // exclude - will not have get nor set method
    #[only="set"] age: u8,          // support only set method 
    division: Division              // support get and set method
}

fn main() {
    let mut s = Sample::new(12, "Fika".to_string(), 27, Division::Finance);

    // ===valid
    let _id = s.get_id();
    s.set_age(30);
    s.set_division(Division::IT);
    let _div = s.get_division();

    // ===invalid
    // s.set_id(58);
    // s.set_name("Tino".to_string());
    // let name = s.get_name();
    // let age = s.get_age();
}

