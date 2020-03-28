pub fn run() {

    // Simple If-Else ladder
    let number = 6;
    if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 3 or 2");
    }

    // If-Else is an Expression and hence can be assigned
    // The if and else arms should have compatible return value types
    let some_condition = false;
    let something = if some_condition {
        println!("Condition was True");
        8 as i128
    } else {
        println!("Condition was False");
        9
    };
    println!("Something is {}", something);
    
    // Loops are also Expressions and hence can return values
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result: {}", result);

    let a = [1,2,3,4];
    // Iterating using while loop on index.. Mutable collections can give rise to bugs
    let mut index = 0;
    while index < a.len() {
        print!("{} ", a[index]);
        index += 1;
    }
    println!();
    // Iterating using for loop on index.. Mutable collections can give rise to bugs
    for index in (0..a.len()).rev() {
        print!("{} ", a[index]);
    }
    println!();
    // Iterating using for loop.. Mutable collections will be iterated safely
    for element in a.iter().rev() {
        print!("{} ", element);
    }
    println!();

    // Solving Practice problems
    println!("{}", practice::temp_converter(false, -40.) );
    println!("{}", practice::get_fibonacci_number(6) );
    
}

mod practice {
    pub fn temp_converter(c_to_f:bool, temp_val:f64) -> f64{
        if c_to_f {
            temp_val/5.*9. + 32.
        } else {
            (temp_val-32.)/9.*5.
        }
    }
    
    pub fn get_fibonacci_number(n: i32) -> i32 {
        if n<=0 { -1 }
        else if n==1 { 0 }
        else if n==2 { 1 }
        else {
            get_fibonacci_number(n-1) + get_fibonacci_number(n-2)
        }
    }
}
