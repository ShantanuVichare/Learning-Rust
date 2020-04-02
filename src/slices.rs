use std::any::type_name;

// Can be ignored for now
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}


pub fn run(){

    // Check types of various string objects
    println!("Type of String Object: {}", type_of( String::from("Boo") ));
    println!("Type of String Slice: {}", type_of( &String::from("Boo")[..] ));
    println!("Type of String Literal: {}", type_of("Boo"));
    // Basically, String Literals are String Slices and represented by &str


    // s1 is a String Object
    // s2 is a String Slice which is an Immutable Refernce to the Object
    let s1 = String::from("hello world");
    let s2 = &s1[2..8];
    println!("s1: {}", s1);
    println!("s2: {}", s2);

    // String objects cannot be automatically casted to String Slices
    // println!("s1 first word: {}", get_first_word(s1)); => Also note that this will cause s1 to lose ownership and go out of scope - will cause problems if s1 or s2 is used later
    // However, String object references CAN be automatically casted to String Slices
    println!("s1 first word: {}", get_first_word(&s1));

    // String slices and their references can be automatically casted to String Slices
    println!("s2 first word: {}", get_first_word(s2));
    println!("s2 first word: {}", get_first_word(&s2));
    
    // Memory locations os string object and string slice
    println!("String object Address: {:?}", s1.as_ptr());
    println!("String slice Address: {:?}", s2.as_ptr());

    // You can pass variables by Assignment or by passing to Functions
    // Heap allocated variables(s1 below) are Dropped when passed and hence not implicitly Copied
    // Stack variables(s2 below) are not dropped when passed instead they are implicitly Copied 
    
    // Assignment types:
    // "let a = b;" OR "b;" => Both statements are valid assignments

    let _ = s2; // Assignment to none - Shallow copies to none // Similar to "s2;"
    // OR some_function(s2); // Passes to function - Shallow copies to function
        /******* Here s2 is still valid *******/

    s1; // Assignment to none - Moves Object/Passes Ownership to none and thus s1 becomes invalid
    // OR some_function(s1); // Assignment to function - Moves Object/Passes Ownership to function and thus s1 becomes invalid
        /******* Now s2 is invalid *******/

    
    let mut s1 = String::from("hello world");
    let s2 = &s1[2..8];
    // Accessing s2 after modifying s1 will cause Unexpected behaviour
    s1.clear();
    // println!("s2: {}", s2); // s2 was expected to be Immutable but as s1 modified the Object, it will throw an error
    


    let arr = [1, 2, 3, 4, 5];
    let array_slice = &arr[1..3];
    println!("Type of Array: {}", type_of( arr ));
    // Here arr is not invalidated because it is a Stack variable and hence can be copied without dropping, thus array_slice will be valid!
    println!("Type of Array slice: {}", type_of( array_slice ));

}

fn get_first_word(s: &str) -> &str {
    let byte_str = s.as_bytes();
    
    for (i,&item) in byte_str.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
