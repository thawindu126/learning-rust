fn main() {
    fahrenheit_to_celsius();
    println!();
    fibonacci();
    println!();
    lyrics_to_twelve_days_of_christmas();
}

fn accept_input<T: std::str::FromStr>() -> std::result::Result<T, T::Err> {
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read your input.");

    return input.trim().parse::<T>();
}

fn fahrenheit_to_celsius() {
    println!("Fahrenheit to Celsius");
    println!("Enter your Fahrenheit value:");
    let fahrenheit_input = accept_input::<f32>().expect("Your input was invalid.");

    let celsius = (fahrenheit_input - 32.0) * (5.0 / 9.0);

    println!("Celsius: {}", celsius)
}

fn fibonacci() {
    println!("Generate nth Fibonacci number");
    println!("Enter your nth value:");
    let nth_input = accept_input::<u32>().expect("Your input was invalid.");

    fn fibonacci_generator(n: u32) -> u64 {
        if n == 0 {
            return 0;
        } else if n == 1 || n == 2 {
            return 1;
        } else if n == 3 {
            return 2;
        }

        let mut sum = 0;
        let mut last = 0;
        let mut current = 1;
        for _ in 1..n {
            sum = last + current;
            last = current;
            current = sum;
        }

        return sum;
    }

    for number in 0..=nth_input {
        println!("{}", fibonacci_generator(number))
    }
}

fn lyrics_to_twelve_days_of_christmas() {
    println!("Lyrics to the Christmas carol \"Twelve Days of Christmas\"");

    const NUMBER: [&str; 12] = [
        "a", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
        "twelve",
    ];

    const NUMBER_TH: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let mut verse: std::collections::VecDeque<std::string::String> =
        std::collections::VecDeque::new();

    for index in 0..12 {
        let number = NUMBER[index];
        let number_th = NUMBER_TH[index];
        println!();

        match index {
            0 => verse.push_front(format!("{} patridge in a pear tree", number)),
            1 => verse.push_front(format!("{} turtle doves and ", number)),
            2 => verse.push_front(format!("{} French hens, ", number)),
            3 => verse.push_front(format!("{} calling birds, ", number)),
            4 => verse.push_front(format!("{} gold rings, ", number)),
            5 => verse.push_front(format!("{} geese a laying, ", number)),
            6 => verse.push_front(format!("{} swans a swimming, ", number)),
            7 => verse.push_front(format!("{} maids a milking, ", number)),
            8 => verse.push_front(format!("{} ladies dancing, ", number)),
            9 => verse.push_front(format!("{} lords a leaping, ", number)),
            10 => verse.push_front(format!("{} pipers piping, ", number)),
            11 => verse.push_front(format!("{} drummers drumming", number)),
            _ => (),
        };

        println!(
            "On the {} day of Christmas my true love gave to me",
            number_th
        );

        for line in verse.iter() {
            println!("{}", line);
        }
    }
}
