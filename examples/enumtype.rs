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
    let colors = Color::variants();

    for c in colors {
        println!("{c}")
    }

    let blue: Color = "Blue".into();

    match blue {
        Color::Blue => println!("It's blue"),
        _ => println!("Unknown color")
    }
}

