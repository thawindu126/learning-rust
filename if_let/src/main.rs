fn main() {
    example_one();
    example_two();
}

fn example_one() {
    let some_u8_value = Some(0u8);

    // Verbose
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    };

    // Concise
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn example_two() {
    #[derive(Debug)]
    enum MyColor {
        Red,
        Blue,
        Indigo,
    }

    let mut color = MyColor::Red;

    // Verbose
    match color {
        MyColor::Red => color = MyColor::Blue,
        _ => color = MyColor::Indigo,
    };

    // Concise
    if let MyColor::Red = color {
        color = MyColor::Blue;
    } else {
        color = MyColor::Indigo;
    }

    println!("{:#?}", color)
}
