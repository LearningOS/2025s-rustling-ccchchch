fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;  // Destructure the tuple

    println!("{} is {} years old.", name, age);
}
