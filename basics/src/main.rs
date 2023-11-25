use std::{
    collections::{HashMap, HashSet},
    vec,
};

fn main() {
    // this is a comment

    /* This is also a comment, but
    more than one line. */

    // say hello, world

    println!("Hello, World!");

    // Variables

    /* a simple rule: all variables must be used, and all
     * values defined in them must be read at least once.
     */

    let my_string = "this is a text string";
    // my_string = "new value"; // this is a bug, variables are immutable by default
    println!("{my_string}");

    let mut my_mutable_string = "this is a mutable text string";
    println!("{my_mutable_string}");
    my_mutable_string = "new value of text string";
    println!("{my_mutable_string}");

    // my_mutable_string = 6; // this is a bug, rust is statically typed

    let my_string2: &str = "this is another text string";
    println!("{my_string2}");

    let mut my_int = 7;
    println!("{my_int}");
    my_int += 4;
    println!("{my_int}");
    my_int += 1;
    println!("{my_int}");

    /* the integer types are:
     *     x8, x16, x32, x64, x128 and xsize
     *
     * where x can be "i" or "u", signed or
     * unsigned integers respectively.
     */
    let my_int2: i32 = 5;
    println!("{my_int2}");

    /* the float types are f32 and f64
     * The default type is f64
     */
    let mut my_float = 6.5;
    println!("{my_float}");

    // my_float = 6; // this is a bug
    my_float = 6.0;
    println!("{my_float}");

    let my_float2: f64 = 123.456;
    println!("{my_float2}");

    let mut my_bool = false;
    println!("{my_bool}");
    my_bool = true;
    println!("{my_bool}");

    // constants

    // constants must have an explicit type definition
    const MY_CONSTANT: &str = "My Constant";
    // MY_CONSTANT = "New Value for Constant"; // this is a bug
    println!("{MY_CONSTANT}");

    // const MY_CONSTANT_INT: i32 = my_int; // error

    // Flow control

    if my_int == 10 && my_string == "Hola" {
        println!("the value is 10")
    } else if my_int == 11 || my_string == "Hola" {
        println!("the value is 11")
    } else {
        println!("the value is neither 10 nor 11")
    }

    // lists, vectors and arrays

    let mut my_list = vec!["Kevin", "Rodriguez", "@kevin4rb200116"];
    my_list.push("Kevin");
    println!("{my_list:?}");
    println!("{}", my_list[1]);

    // sets

    let mut my_set: HashSet<&str> = vec!["Kevin", "Rodriguez", "@kevin4rb200116"]
        .into_iter()
        .collect();
    my_set.insert("Kevin");
    println!("{:?}", my_set);

    // maps

    let mut my_map: HashMap<&str, i32> = vec![("Kevin", 22), ("Luis", 16)].into_iter().collect();
    my_map.insert("Justin", 15);
    println!("{:?}", my_map);

    // loops

    for value in &my_list {
        println!("{value}");
    }

    for value in &my_set {
        println!("{value}");
    }

    for (key, value) in &my_map {
        println!("{key} {value}");
    }

    let mut my_counter = 0;
    while my_counter < my_list.len() {
        println!("{}", my_list[my_counter]);
        my_counter += 1;
    }

    // functions

    my_function();

    // structs

    let my_struct = User {
        name: String::from("Justin"),
        age: 15,
    };

    println!("{} {}", my_struct.name, my_struct.age);

    let my_struct2 = User::new("Kevin", 22);

    println!("{} {}", my_struct2.name, my_struct2.age);
}

fn my_function() {
    println!("this is a function");
}

struct User {
    name: String,
    age: u8,
}

impl User {
    fn new(name: &str, age: u8) -> Self {
        Self {
            name: String::from(name),
            age,
        }
    }
}
