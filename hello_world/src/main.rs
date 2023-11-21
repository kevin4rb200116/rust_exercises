use std::io;

struct User {
    name: String,
    age: u8,
}

impl User {
    fn new() -> Self {
        println!("Hello, what's your name?");

        let mut user_name = String::new();
        let mut user_age = String::new();

        io::stdin()
            .read_line(&mut user_name)
            .expect("Failed to read line");

        println!("How old are you?");
        io::stdin()
            .read_line(&mut user_age)
            .expect("Failed to read line");

        let user_age: u8 = user_age.trim().parse().expect("Please type a number");

        Self {
            name: user_name,
            age: user_age,
        }
    }
}

fn main() {
    let user = User::new();

    println!("Hello, {}", user.name);
    println!("Wow your are {} years old!", user.age);
}
