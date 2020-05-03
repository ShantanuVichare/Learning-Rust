// Closures are anonymous functions you can save in a variable or pass as arguments to other functions
// Can be defined and called in different contexts

pub fn run() {

    // Closures can infer type and can be optionally type annotated in their definitions
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;
    // However, to infer types, the "add_one_v3" and "add_one_v4" closures must be called
    println!("Closure results: {},{}", add_one_v3(3),add_one_v4(4));


    // Closures are treated as "Types" and it is same as the type of Return value of a closure definition/call
    // Thus Closures implement Traits
    let mut c = Cacher::new(|x| x);
    let three = c.get_value(3); // This call will execute the closure
    let three2 = c.get_value(3); // This call will fetch the pre-calculated value


    // Cacher2 will reimplement caching to allow storing several values while each object can store different type
    let mut int_c = Cacher2::new(|x| x);
    let three = int_c.get_value(&3); // This call will execute the closure
    let three2 = int_c.get_value(&3); // This call will fetch the pre-calculated value
    let two = int_c.get_value(&2);

    let mut string_c = Cacher2::new(|x| x);
    let apple = String::from("Apple");
    let banana = String::from("Banana");
    let a_first_call = string_c.get_value(&apple);
    let a_second_call = string_c.get_value(&apple); // Reading from HashMap
    let b_first_call = string_c.get_value(&banana);
    println!("a_first_call: {}\na_second_call: {}\nb_first_call: {}", a_first_call, a_second_call, b_first_call);

    let mut len_c = Cacher2::new(|x:String| x.len());
    let apple = String::from("Apple");
    let banana = String::from("Banana");
    let a_first_call = len_c.get_value(&apple);
    let a_second_call = len_c.get_value(&apple); // Reading from HashMap
    let b_first_call = len_c.get_value(&banana);
    println!("a_first_call: {}\na_second_call: {}\nb_first_call: {}", a_first_call, a_second_call, b_first_call);
    

    // Closures can capture values from the environment in which they’re defined
    let mut apple = String::from("Apple");
    
    let is_Apple = |input_str: &str| input_str==apple; // "is_Apple" Closure borrows "apple" immutably
    assert!(is_Apple("Apple"));

    let mut is_Apple2 = |input_str: &str| {
        apple = String::from("apple");
        input_str==apple
    }; // "is_Apple" Closure borrows "apple" mutably
    assert!(is_Apple2("apple"));
    
    let is_Apple3 = move |input_str: &str| input_str==apple; // "is_Apple" Closure forcefully takes ownership of "apple"
    // "apple" is now invalid
    assert!(is_Apple3("apple"));

    // Depending on how Closures capture the Closure's Environment variables, they implement the Fn Traits
    // FnOnce - Values move into the Closure's scope
    // FnMut - Values borrowed Mutably
    // Fn - Values borrowed Immutably

}

// All Closures implement one of Fn, FnMut, FnOnce traits
// These special traits have input and output parameter types
struct Cacher<T> 
where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}
// Cacher struct will calculate(execute the Closure) or return the value if it's already calculated
impl<T> Cacher<T>
where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {calculation,value: None}
    }
    fn get_value(&mut self, arg: u32) -> u32 {
        match self.value{
            Some(v) => v,
            None => {
                self.value = Some((self.calculation)(arg));
                self.value.unwrap()
            }
        }
    }
}


// Cacher2 implementation - overcoming Cacher limitations
use std::collections::HashMap;
use std::hash::Hash;

struct Cacher2<T,U,V>
where T: Fn(U) -> V, U: Hash+Eq+PartialEq+Clone, V:Clone {
    calculation: T,
    map: HashMap<U,V>,
}
// Cacher 2 will store to a HashMap
impl<T,U,V> Cacher2<T,U,V>
where T:Fn(U) -> V, U: Hash+Eq+PartialEq+Clone, V:Clone {
    fn new(calculation: T) -> Cacher2<T,U,V> {
        Cacher2 {calculation, map: HashMap::new()}
    }
    fn get_value(&mut self, arg: &U) -> V {
        match self.map.get(arg) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg.clone());
                self.map.insert(arg.clone(), v.clone());
                v
            }
        }
    }
}
