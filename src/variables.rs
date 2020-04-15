pub fn run() {

    // Variable type and casting`
    let quotient:i32 = 50/7;
    println!("{}", quotient as f64/2 as f64);

    // Tuple
    let tup = (500,6.4,1);
    let (x, _y, _z) = tup;
    println!("{},{}", x,tup.2);

    // Array
    // All below initializations are equivalent
    let mut a = [3,3,3,3];
    let mut a = [3;4];
    let mut a:[i32;4] = [3;4];
    a[0] = 5;
    println!("a = [{},{},{},{}]",a[0],a[1],a[2],a[3]);

    println!("Type of a: {}", type_of(a));

}
// Ignore below
use std::any::type_name;
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}