use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub fn run(){

    // Running code for each collection types
    // vectors::run();
    // strings::run();
    hashmaps::run();
}

mod vectors{
    // Bringing 'type_of' function into current scope from parent scope
    use super::type_of;
    pub fn run(){
        println!("\nVector run....\n");

        // Defining a vector without inserting any values hence passing parameter type Generic
        let v1: Vec<i32> = Vec::new();
        println!("v1: {:?}", v1);
        // Defining and initialising values using vec! macro with auto type inference
        let v2 = vec![1,2,3];
        println!("v2: {:?}", v2);
        println!("Types: v1: {}, v2: {}", type_of(v1),type_of(v2));
        // v1 and v2 have moved and hence invalidated


        // Mutable vectors
        // If the type of values that will be pushed can be inferred at compile time, Rust will allow declaration without type checking
        let mut v = Vec::new();
        v.push(5);
        v.push(4);
        v.push(3);

        // Both ways we get a reference to the third element
        let third = &v[2]; // Code can panic here if index is out of bounds
        let third = match v.get(2){ // Get returns an Option enum
            Some(third) => third,
            None => &(-1),
        };
        println!("Type of third: {}", type_of(third));
        
        // Since, 'third' was an immutable borrow after modifying 'v', 'third' will be invalidated here
        v.push(8);
        // Pop also Returns an Option enum
        println!("Popped: {:?}", v.pop());
        
        // Iterating over a vector
        for i in &mut v{
            // * Dereference operator is used to get the value back
            let temp = *i;
            *i *= 10;
            print!("{} -> {}", temp, i);
            println!(" Type: {}", type_of(i));
        }

        // Iterating simultaneously over multiple vectors using Zip
        let v1 = v; // v has been moved
        let v2 = vec![0.5,0.4,0.3];
        for (i1,i2) in v1.iter().zip(v2.iter()) {
            println!("i1: {} i2: {}", i1, i2);
        }

        // To create a vector of different parameter types and known at compile time
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(2),
            SpreadsheetCell::Text(String::from("Boo")),
            SpreadsheetCell::Float(9.3)
        ];


        // Advanced Iterators
        let mut v1 = vec![
            String::from("string 1"),
            String::from("string 2"),
            String::from("string 3"),
        ];
        for i in v1.iter() {
            // i will be an Immutable borrow
        }
        for i in v1.iter_mut() {
            // i will be a Mutable borrow
        }
        for i in v1.into_iter() {
            // i will take ownership of the Values
        } // v1 will have been moved and invalidated now
        
        
        // Using extend to manipulate vectors
        let mut v1 = vec![1,2,3];
        let v2 = vec![4,5,6];
        v1.extend(v2.iter()); // Extend will borrow v2 immutably
        // v2 is valid
        v1.extend(v2); // Extend will automatically call into_iter() effectively moving v2
        // v2 is invalid

        println!("v1: {:?}", v1);

    }
}

mod strings{
    use super::type_of;
    pub fn run(){
        println!("\nString run....\n");

        // String and Vector types have many similar methods
        let mut s1 = String::new();
        s1.push('F'); // Single character is appended here

        // Strings can be appended
        s1.push_str("oo");
        s1 += "Bar";
        
        // Strings can also be derived from to_string() method which is available for Display trait
        let s2 = "FooBar".to_string();
        println!("s1: {} s2: {}", s1,s2);
        println!("Type of s1: {}\nType of s2: {}", type_of(s1), type_of(s2));


        // Concatenation requires passing of variables
        let s1 = String::from("Hello");
        let s2 = String::from("World");
        let s3 = s1 + &s2; // + operator calls the add method which takes self and &str
        // Here, s1 has been moved and hence invalidated, whereas s2 wasn't moved and hence remains valid
        println!("s3: {}", s3);

        // For complex concatenation, its easier to use format! macro
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        // Macros do not move values and hence variables do not get invalidated
        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("s: {}", s);
        

        // Strings do not allow single character indexing like s[2] as Unicode can have characters of varied byte lengths
        // However, Strings can be indexed over a range of 'Bytes' i.e. a String slice. But this can lead to code panicking if the slice breaks a Unicode scalar (multi-byte character)

        // Strings can be iterated over Unicode Characters
        for c in "नमस्ते".chars() {
            println!("{}", c);
        }
        // Strings can be iterated over Raw Bytes
        for b in "नमस्ते".chars() {
            println!("{}", b);
        }
        

    }
}

mod hashmaps {
    use super::type_of;
    use std::collections::HashMap;
    pub fn run(){
        println!("\nHashMap run....\n");

        // Defining a HashMap 
        let mut scores = HashMap::new();
        // Adding elements
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        println!("scores: {:?}", scores);
        // Removing elements using Reference to Key object
        if scores.contains_key(&String::from("Blue")) {
            scores.remove(&String::from("Blue"));
        }
        println!("scores: {:?}", scores);
        println!();
        
        // HashMap operations return an Option Enum with Value object 
        println!("{:?}", scores.insert(String::from("Blue"),20)); // scores: {"Blue": 20, "Yellow": 50}
        println!("{:?}", scores.insert(String::from("Blue"),80)); // scores: {"Blue": 80, "Yellow": 50}
        println!("{:?}", scores.remove(&String::from("Blue")));   // scores: {"Yellow": 50}
        println!("{:?}", scores.remove(&String::from("Green")));  // scores: {"Yellow": 50}
        println!();

        
        // HashMap can be constructed using .collect() on vector of tuples
        // Type annotation is required as .collect() can be used to construct various data structures
        let teams  = vec![String::from("Blues"), String::from("Reds")];
        let scores = vec![2, 3];
        println!("Type of Tuple vector: {}", type_of(teams.iter().zip(scores.iter())) );
        let score_map: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();
        println!("score_map: {:?}", score_map);
        println!();

        
        // HashMap takes ownership of Keys and Value objects
        let mut map = HashMap::new();
        let field_key = String::from("Color");
        let field_value = String::from("Blue");
        map.insert(field_key, field_value);
        // 'field_key' and 'field_value' have moved into 'map' and are invalidated
        let field_key = String::from("Shape");
        let field_value = String::from("Circle");
        map.insert(field_key.clone(), field_value.clone());
        // Clones of 'field_key' and 'field_value' have moved into 'map' and hence remain valid
        
        println!("field_value: {:?} and its type: {}", map.get(&field_key), type_of(map.get(&field_key)));

        // Iterating over HashMap
        for (key,value) in &map { // Iterators will borrow the values
            println!("{} => {}", key, value);
        }
        for (key,value) in map { // Iterators will take ownership of values
            println!("{} => {}", key, value);
        } // Now map is moved and hence invalidated
        println!();

        
        // Using Entry enum for additional functionalities
        let mut counts: HashMap<String,i32> = HashMap::new();

        for word in "hello world beautiful world".split_whitespace() {
            // .entry(Key) will return an Entry enum
            // .or_insert(Value) will put Value for Key if and only if Key doesn't exist
            // .or_insert() will return &mut for the Value object
            let count = counts.entry(word.to_string()).or_insert(0);
            *count += 1;
        }
        println!("counts: {:?}", counts);

        // Note: Mutable borrows to Key/Value objects are Mutable borrows of the HashMap too as per ownership rules
        let hello_ref = counts.entry("hello".to_string()).or_insert(0);
        // 'hello_ref' is a Mutable borrow of counts and hence cannot be borrowed (mutable/immutable) as long as 'hence_ref' is valid
        // println!("counts: {:?}", counts); // The print will attempt to borrow 'counts' immutably and hence violate ownership rules
        println!("hello_ref: {}", hello_ref); // This will ensure the lifetime of 'hello_ref' till this line


        // User-defined data structures can be used as Keys by deriving Hash, Eq, PartialEq traits
        #[derive(Hash, Eq, PartialEq, Debug)]
        struct Person {
            name: String,
            age: usize,
        }

        let mut country_map: HashMap<Person,String> = HashMap::new();

        country_map.insert(Person{name:"Adam".to_string(),age:25}, "USA".to_string());
        country_map.insert(Person{name:"Ajay".to_string(),age:32}, "India".to_string());

        println!("country_map: {:#?}", country_map);

    }
}