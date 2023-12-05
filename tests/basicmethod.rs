use basicmethod::BasicMethod;

#[derive(BasicMethod)]
/// A struct to create dog, cat or others animal
struct Animal {
    #[only="get"] name: String,
    #[only="set"] age: u16,
    sound: String,
}

#[test] fn test1() {
    let dog = Animal::new("Micho".into(), 8, "Woof".into());

    assert_eq!("Micho", dog.get_name().as_str());
    assert_eq!("Woof", dog.get_sound().as_str());
}

#[test] fn test2() {
    let mut dog = Animal::new("Micho".into(), 8, "Woof".to_string());
    
    dog.set_age(15);
    dog.set_sound("goof".into());

    assert_eq!("Micho", dog.get_name().as_str());
    assert_eq!(15, dog.age);
    assert_eq!("goof", dog.get_sound().as_str());
    assert_eq!("A struct to create dog, cat or others animal", Animal::info());
}


#[derive(BasicMethod)]
/// Unit struct
struct Unit;

#[test] fn test3() {
    assert_eq!("Unit struct", Unit::info());
}

#[derive(BasicMethod)]
/// Tuple struct
struct User(u32, String);

#[test] fn test4() {
    let mut user = User(11, "Robert".to_string());

    assert_eq!("Tuple struct", User::info());
    assert_eq!(11u32, *user.get_u32_0());
    
    user.set_String_1("William".to_string());
    assert_eq!("William", user.get_String_1());
}

