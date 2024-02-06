fn main() {
    let number = 42;

    match number {
        x if x > 0 => println!("The number is positive"),
        x if x < 0 => println!("The number is negative"),
        _ => println!("The number is zero"),
    }
}
