pub fn run() {

    // Variable type and casting`
    let quotient:i32 = 50/7;
    println!("{}", quotient as f64/2 as f64);

    // Tuple
    let tup = (500,6.4,1);
    let (x, _y, _z) = tup;
    println!("{},{}", x,tup.2);

    // Array
    let mut a:[i32;4] = [3;4];
    a[0] = 5;
    println!("{}",a[0]);
}
