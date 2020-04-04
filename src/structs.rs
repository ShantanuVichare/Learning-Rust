pub fn run(){

    // Struct Instantiation directly
    let mut user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("someone123"),
        active: true,
        sign_in_count: 1
    };
    // To mutate any field, entire struct instance needs to be declared mutable
    user1.email = String::from("anotherone@example.com");
    // Debug printing for User struct with derives Debug trait
    println!("user1: {:?}", user1);

    // Struct Instantiation using a constructor function
    let user2 = build_user(String::from("sometwo@example.com"),String::from("sometwo456"));
    println!("user2: {:?}", user2);

    // Struct Instantiation using Struct Update Syntax
    let user3 = User{
        email: String::from("somethree@example.com"),
        username: String::from("somethree789"),
        ..user1
    };
    println!("user3: {:?}", user3);


    // Tuple structs: similar to tuples but each of them are their own types like structs
    // Ex: Color and Point are similar in structure but are treated as seperate parameter types for functions,etc
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black: {:?}", black); // Will work because Debug trait is derived for Color type
    // println!("{:?}", origin); // Will give error because Debug trait is not derived for Point type
    println!("origin: {2},{0},{1}", origin.1, origin.2, origin.0);

}

// Example Struct definition
#[derive(Debug)] // Deriving Debug trait for printing User struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
    // some_other_field: &str // To add references as fields we need lifetimes which will be discussed later
}

fn build_user(email: String, username: String) -> User {
    User {
        // Field init shorthand syntax since parameter names and field names are same
        email,
        username,
        sign_in_count: 1,
        active: true
    }
}

#[derive(Debug)] // Deriving Debug trait for printing Color tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
