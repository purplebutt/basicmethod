use basicmethod::BasicMethod;

#[derive(BasicMethod)]
struct User(u32, String, String, i32);

fn main() {
    let mut user = User(23, "Mika".to_string(), "London".to_string(), 10);

    user.set_String_1("Lidia".to_string());
    user.set_String_2("New York".to_string());
    println!("{}", user.get_String_1());
    println!("{}", user.get_String_2());

    user.set_u32_0(2);
    user.set_i32_3(7);
    println!("{}", user.get_u32_0());
    println!("{}", user.get_i32_3());

    for f in User::fields() {
        println!("{}", f)
    }
}

