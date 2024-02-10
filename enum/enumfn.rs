#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}


fn is_weekend(day: Day) -> &'static str {
    match day {
        Day::Saturday | Day::Sunday => "It's the weekend!",
        _ => "It's a weekday.",
    }
}

fn main() {
    let today = Day::Monday;
    let weekend = Day::Saturday;

    println!("{}", is_weekend(today));
    println!("{}", is_weekend(weekend));
}
