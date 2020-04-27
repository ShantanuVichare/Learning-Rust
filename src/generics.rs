// This file will cover concepts of Generics, Traits and Lifetimes
// 
// Generics allow implementing the same code for various types
// Traits allow to define behaviour(methods and operations) for various types(incl Generics)
// Lifetimes allow to define the validity scope for types(incl Generics) 
// 

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

pub fn run() {
    generics::run();
    traits::run();
    lifetimes::run();
}

mod generics {
    struct Point<T> {
        x: T,
        y: T,
    }

    // Implementation block for a struct with Generics requires the same Generics to be passed to 'impl'
    impl<T> Point<T> {
        fn get_x(&self) -> &T {
            // Since we haven't defined the Copy Trait to T, we have to return a reference
            &(self.x)
        }
    }

    // Implementation for a specific Generic type can be done
    // Generic implementations should not conflict with the signatures with type-specifc implementation
    impl Point<f32> {
        fn dist_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
        fn has_type(&self) -> String {
            "Float".to_string()
        }
    }
    // However if types are not clashing, we can have same function signatures
    impl Point<i32> {
        fn has_type(&self) -> String {
            "Integer".to_string()
        }
    }

    pub fn run() {
        println!("--Generics--");
        // Generic definitions take type argument
        let integer_pt = Point { x: 3, y: 4 }; // Auto-type inference
        let mut float_pt: Point<f32>; // Explicit type annotation
        // float_pt = Point{x:2.3, y:3.4}; // Since explicit type annotation is done, we need not initialise 'float_pt'
        println!("X value of Int Point: {}", integer_pt.get_x());

        let float_pt2 = Point { x: 2.3, y: 3.4 };
        println!("Dist from origin: {}", float_pt2.dist_from_origin());

        println!("Type of integer_pt: {}", integer_pt.has_type());
        println!("Type of float_pt: {}", float_pt2.has_type());
    }
}

mod traits {

    // Traits are similar to Interfaces with some differences
    pub trait Summary {
        // Types implementing Summary trait will have to implement summarize1() as per the following signature
        fn summarize1(&self) -> String;

        // Its not mandatory to implement summarize2() and the following will be the default implementation unless overridden
        fn summarize2(&self) -> String {
            String::from("Default summary")
        }
    }

    struct Rectangle {
        height: u32,
        width: u32,
    }
    // Implementing a Trait for a Type
    impl Summary for Rectangle {
        // Implementing summarize1
        fn summarize1(&self) -> String {
            format!(
                "Rectangle with height: {} and width: {}",
                self.height, self.width
            )
        }
        // Overiding default summarize2 implementation => The only overidding possible 
        fn summarize2(&self) -> String {
            format!("Rectangle is {} tall and {} wide", self.height, self.width)
        }
    }

    struct Square {
        side: u32,
    }
    impl Summary for Square {
        // Implementing summarize1
        fn summarize1(&self) -> String {
            format!("Square with side: {}", self.side)
        }
        // Proceeding with the default summarize2 implementation
    }


    use std::fmt::Display;
    // Functions can accept Trait bounded parameters for being able to utilize Trait defined methods
    // The following 3 definitions are identical in behaviour
    fn notify1 (item1: impl Summary, item2: impl Summary + Display) {}

    fn notify2<T: Summary, U: Summary + Display> (item1: T, item2: U) {}

    fn notify3<T,U> (item1: T, item2: U) // -> Return_Type_Here
    where T: Summary, U: Summary + Display {}

    // We can also return types which implement a given Trait
    fn returns_summarizable<T: Summary> (item: T) -> impl Summary { item }

    // Using generics and traits to define a function to get the max value in a Array
    fn largest<T: PartialOrd + Copy> (list: &[T]) -> T {
        // PartialOrd allows for comparison operators of T objects
        // Copy allows for implicit copying of T objects
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    // Types implementing specific traits can have specific Method implementations while overiding the default methods
    struct MyType<T> {x:T}
    impl<T: Display> MyType<T> {
        fn can_display(&self) -> bool {true}
    }
    impl<T> MyType<T> 
    where T: Copy {
        fn can_copy(&self) -> bool {true}
    }

    pub fn run() {
        println!("--Traits--");

        // Implementing Traits and overiding default method definitions
        let rect = Rectangle {
            height: 30,
            width: 50,
        };
        let sq = Square { side: 40 };
        println!(
            "Summaries:\n\t{}\n\t{}\n\t{}\n\t{}",
            rect.summarize1(),
            rect.summarize2(),
            sq.summarize1(),
            sq.summarize2()
        );

        // Methods implementing Generics need Trait implementations to even allow basic operators (behaviours)
        let number_list = vec![1,2,43,4,25];
        let char_list = String::from("Apple");
        println!("Largest number:{}\nLargest char:{}", largest(&number_list), largest(&char_list.as_bytes()) as char);

        // Testing methods for conditional Traits 
        let str_val = MyType{x: String::from("")};
        let int_val = MyType{x: 0};
        str_val.can_display();
        // str_val.can_copy(); // This will give error as String doesn't implement Copy trait
        int_val.can_display();
        int_val.can_copy();
    }


}

mod lifetimes {

    // Lifetimes ensure validity of scopes and hence avoid Dangling references
    // A mutable reference with an explicit lifetime is declared as: &'a mut i32
    // Lifetime Specifiers are nothing but Generics but used for a specific purpose

    fn longer_string<'a> (x: &'a str, y: &'a str, useless_arg: &str) -> &'a str {
        
        // The actual lifetime defined by 'a will be the shorted of these two
        // Arguments which have no relationship with the return value need not have a lifetime specifier
        // All lifetime arguments must have some relation with the lifetimes of the input parameters

        if x.len() > y.len() {x}
        else {y}
    }

    // Structs can hold references only using lifetimes!
    #[derive(Debug)]
    struct MyContainer<'a,T> {
        value: &'a mut Vec<T>,
    }

    // Combining Generics, Traits and Lifetimes
    impl<'a,T> MyContainer<'a,T> 
    where T: std::fmt::Display {
        fn add_element_and_get(&mut self, element: T) -> & Vec<T> { // Lifetime Elision Rules 1 & 3
            self.value.push(element);
            &self.value
        }
    }

    fn same_as(inp: &str) -> &str { // Lifetime Elision Rules 1 & 2
        inp
    }

    pub fn run(){
        println!("--Lifetimes--");

        // Lifetimes are defined for values allocated on the Heap, i.e. Values that don't implement Copy trait
        let first = String::from("Hello");
        let result1;
        let result2;

        // Defining a sub-scope/lifetime
        {
            let second = String::from("Bro");

            result1 = longer_string(first.as_str(), second.as_str(), "blablabla");
            result2 = longer_string(second.as_str(), first.as_str(), "blablabla");

            println!("Result 1 is: {}", result1);
            println!("Result 2 is: {}", result2);
        }
        // "second" goes out of scope here but "first" is still valid

        // Although, "result1" and "result2" have results derived from "first", their lifetimes are restricted by "second"
        // Since, we defined lifetime 'a for both the arguments, it defaults to the shorter of the both lifetimes

        // The following lines will throw an error because "result1" and "result2" don't live in this scope
        // println!("Result 1 is: {}", result1);
        // println!("Result 2 is: {}", result2);


        // Storing a reference to a Vec in struct 
        let mut v = vec![1,2,3];
        let mut cont1 = MyContainer{value: &mut v};
        println!("cont1 is: {:?}", cont1);


        // Lifetime Inference can be done in some special cases using Lifetimes Elision Rules
        // Rule 1: Each of the input params gets an individual lifetime

        // Rule 2: Output lifetime parameter can be inferred if there is only 1 input lifetime param
        let temp = same_as("Wow");
        println!("Value:{:?}, Type: {}", temp, super::type_of(&temp));
        // Rule 3: For methods, the return value gets the lifetime of "&self"
        let temp = cont1.add_element_and_get(4);
        println!("Value:{:?}, Type: {}", temp, super::type_of(&temp));

        // 'static lifetime lasts throughout the duration of the program
        // super::type_of(_: &T) returns a value with 'static lifetime
        // String literals are defined at compile time and hence have a 'static lifetime 
    }
}