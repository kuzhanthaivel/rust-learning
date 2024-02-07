fn main() {
    let mut count = 0;

    loop {
        println!("Current count: {}", count);

        if count >= 5 {
            break; 
        }

        count += 1;
    }
}
