#[derive(Debug)] // Derive the Debug trait for the Day enum
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn main() {
    let today = Day::Monday;
    let weekend = Day::Saturday;


    println!("Today is {:?}", today);
    println!("Weekend is {:?}", weekend);
}
