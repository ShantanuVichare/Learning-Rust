use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub fn run() {

    // Void Return function call
    show(5, 6);

    // Statements don't return anything expressed by an empty tuple '()'
    // Expressions return values
    // 
    // Adding a ';' to an Expression makes it a Statement
    // { } Evaluate to the last Expression inside them or an () if there aren't any
    let x = 8;
    println!("Return Type of 'x' or '{{x}}': {}", type_of({x}));
    println!("Return Type of 'x;' or '{{x;}}': {}", type_of({x;}));

    // Returning value from different expressions 
    println!("{}, {}", do_something(4, 9),do_something(4, 8));

}

fn show(x:i32, y:i32){
    println!("x={}, y={}", x,y);
}

fn do_something(x:i32, y:i32) -> i32 {
    if y%2==0 {
        x+y
    }
    else{
        x-y
    }
}