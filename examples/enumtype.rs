use basicmethod::BasicMethod;

#[allow(unused)]
#[derive(BasicMethod)]
enum Color {
    // #[exclude]
    // #[only]
    Red,
    Green,
    Blue
}


fn main() {
    let _color = Color::Red;
    let x = Color::variants();

    for i in x {
        println!("{i}")
    }

    let blue: Color = "Blue".into();

    match blue {
        Color::Blue => println!("It's blue"),
        _ => println!("Unknown color")
    }
}

