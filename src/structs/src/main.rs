#[derive(Debug)]
struct Person{
    name: String,
    age: u8,
    likes_oranges: bool

}
fn main() {
    let person = Person{
        name: String::from("Adam"),
        likes_oranges: true,
        age: 25
    };

    println!("Person name is {}", person.name);
    println!("Person age is {}", person.age);
    println!("{:?}", person);
}
