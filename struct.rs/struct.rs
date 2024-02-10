
struct Person {
    name: String,
    age: u32,
    is_student: bool,
}

fn main() {

    let alice = Person {
        name: String::from("Alice"),
        age: 30,
        is_student: false,
    };


    println!("Name: {}", alice.name);
    println!("Age: {}", alice.age);
    println!("Is student: {}", alice.is_student);
}
