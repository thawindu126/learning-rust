fn main() {
    let s1 = gives_ownership();

    takes_ownership(s1);

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    // COMPILE_ERROR
    // -> println!("{} {} {}", s1, s2, s3);
    println!("{}", s3);

    multiple_references();

    let s4 = no_dangle();
    println!("{}", s4);

    // -------------------------------------
    let s5 = "Hello, world!";
    println!("{}", first_word(&s5));
    println!("{}", second_word(&s5));
}

fn takes_ownership(a_string: String) {
    println!("{}", a_string);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn multiple_references() {
    let mut s = String::from("hello");

    // Multiple immutable references are valid.
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    // Valid, because the last usage of r1 and r2
    // occurs before r3 is introduced.
    let r3 = &mut s;

    // COMPILE_ERROR - Mutable and immutable references cannot coexist in the same scope.
    // -> println!("{}, {}, and {}", r1, r2, r3);
    println!("{}", r3);

    // Valid, because the last usage of r3
    // occurs before r4 is introduced.
    let r4 = &mut s;

    // COMPILE_ERROR - Multiple mutable references cannot coexist in the same scope.
    // -> println!("{} and {}", r3, r4);
    println!("{}", r4)
}

// fn dangle() -> &String
fn no_dangle() -> String {
    let s = String::from("hello");

    // &s
    s
}

// --------------------------------------------------------------------------------------------
// SLICING

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    let mut first_space = 0;
    let mut second_space = s.len();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if first_space == 0 {
                first_space = i;
            } else if second_space == s.len() {
                second_space = i
            } else {
                break;
            }
        }
    }

    &s[(first_space + 1)..second_space]
}
