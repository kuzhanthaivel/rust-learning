fn main() {
    let signed_integer: i32 = -42;
    let unsigned_integer: u64 = 42;

    let float_number: f64 = 3.14;
 
    let is_true: bool = true;

    let my_char: char = 'A';

    let my_tuple: (i32, f64, char) = (42, 3.14, 'A');

    let my_array: [i32; 3] = [1, 2, 3];

    let my_string: String = String::from("Hello, Rust!");

    println!("The value of signed_integer is: {}", signed_integer);
    println!("The value of unsigned_integer is: {}", unsigned_integer);
    println!("The value of float_number is: {}", float_number);
    println!("The value of is_true is: {}", is_true);
    println!("The value of my_char is: {}", my_char);
    println!("The value of my_tuple is: {:?}", my_tuple);
    println!("The value of my_array is: {:?}", my_array);
    println!("The value of my_string is: {}", my_string);
    
}
