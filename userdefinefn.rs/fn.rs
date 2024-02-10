fn main() {
   
    greet("Alice");
    println!("3 + 4 is {}", add(3, 4));
    println!("Factorial of 5 is {}", factorial(5));

    let add_one = |x| x + 1;
    println!("5 plus 1 is {}", add_one(5));
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

