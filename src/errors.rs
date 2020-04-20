use std::fs::File;
use std::fs::remove_file;
use std::io;

pub fn run(){

    // Result enum is used for most of the Error Handling in Rust
    // The definition of Result Enum is as follows:
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    
    // Result can be Unwrapped directly to T without error checking.. Program will panic! in case of Err
    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("hello.txt should exist");

    // Result can be unwrapped to E but Program will panic! if its Ok.. useful for debugging
    let f1 = File::open("hello.txt").unwrap_err();
    let f2 = File::open("hello.txt").expect_err("hello.txt already exist!");
    println!("Types after unwraps: {}, {}", type_of(&f1),type_of(&f2));
    
    // Proper error handling can be done using nested match statements
    let f = match File::open("hello.txt") {
        Ok(opened_file) => opened_file,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(created_file) => created_file,
                Err(e) => panic!("Problem creating the file: {:?}",e),
            },
            some_other_error => panic!("Problem opening the file: {:?}",some_other_error),
        },
    };
    println!("Type of f: {}", type_of(&f));

    // Errors can also be handled using 'unwrap_or_else' method on the Result enum
    // 'unwrap_or_else' method takes a Closure which can return a T object
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == io::ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });
    // println!("Type of f: {}", type_of(&f));


    // Error Propagation can be achieved by returning a Result enum
    println!("Type of first read: {}", type_of(&read_file1()));
    // To return the Err(E) Result enum has a special ? operator
    println!("Type of second read: {}", type_of(&read_file2()));
    

    // Removing the file for repeatable results
    remove_file("hello.txt").expect("File does not exist!");
}

// Error Propagation from functions
fn read_file1() -> Result<String, io::Error> {
    // Read trait allows using functions required to read from Files
    use std::io::Read;
    
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_no_of_bytes) => Ok(s),
        Err(e) => Err(e),
    }
}

// Using the ? operator for Error Propagation
fn read_file2() -> Result<String,io::Error> {
    use std::io::Read;

    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}


// Function to get type of variables from their references
fn type_of<T>(_:&T) -> &'static str {
    std::any::type_name::<T>()
}