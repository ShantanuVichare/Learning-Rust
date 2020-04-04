
use std::any::type_name;
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}


// Enum's value can only be one of its variants
enum IpAddrKind {
    // Unit-like variants
    V4,
    V6,
}

// Enums can store data too
enum IpAddr {
    // Struct tuple variants
    V4(u8,u8,u8,u8),
    V6(String),
}

// Enum with varied type of variants
enum Message {
    // Unit-like variant
    Quit,
    
    // Struct variant
    Move { x: i32, y: i32 },

    // Struct tuple variant
    Write (String),
    // A different Struct tuple variant
    ChangeColor(i32, i32, i32),

    // Useless variants
    Random1,
    Random2(i32),
}

impl Message {

    // Self will be reference to the enum instance and hence will be one of the variants
    fn call(&self) -> i32 {
        // Match will do pattern matching on the variant types 
        match self{
            Message::Quit => 0,
            Message::Move{x:a,y:b} => a+b,
            Message::ChangeColor(a,b,c) => a+b+c,
            Message::Write (text) => {
                text.len() as i32
            },
            _ => -1, // Placeholder for remaining variant types
        }
    }
}


pub fn run(){

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Type of both enum variants is the IpAddrKind enum 'parameter type'
    println!("four: {}", type_of(four));
    println!("six: {}", type_of(six));

    // Different variants can have variant types of different parameter types
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

    // Enums can also implement methods
    let msg = Message::Move{x:3,y:4};
    println!("msg call: {}", msg.call());


    // Option Enum is available in prelude(default imports) and has the foll definition
    // enum Option<T>{
    //     Some(T),
    //     None,
    // }

    // Following are examples of Option<T> Enums
    let some_string = Some("a string");
    let some_number = Some(5);
    let absent_number: Option<i32> = None;
    
    println!("Type of some_string: {} \nType of absent_number: {}", type_of(some_string),type_of(absent_number));
    println!("Addition of some_number: {:?} \nType of some_number: {}", add(some_number,6), type_of(some_number));


    // Using 'match' is exhaustive
    // To execute multiple statements for one specific variant use 'if let' 
    // Adding 'else' block handles for the rest of the variants and is optional
    let some_u8_value = Some(0u8);

    if let Some(3) = some_u8_value {
        println!("Three!");
    }
    // else { /** Placeholder (_) case is handled here **/ }
     
    match some_u8_value {
        // Remember, 'match' performs linear search for Pattern Matching
        Some(3) => println!("Three!"),
        Some(a) => println!("Some u8 Value is {}", a),
        _ => (),
    }


}


fn add(opt: Option<i32>, delta: i32) -> Option<i32> {
    match opt {
        Some(i) => Some(i+delta),
        None => None,
    }
}
