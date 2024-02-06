use std::io::stdin;

fn main(){
    println!("Enter a Number:");
    let mut input = String :: new();
    stdin().read_line(&mut input).expect("Invalid input");
    let number : i32 = input.trim().parse().expect("Error");
    if number%2 == 0{
        println!("The Number {} is Even", number);
    }else{
        println!("The Number {} is Odd", number);
    }
}