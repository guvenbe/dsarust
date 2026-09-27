#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
    children: i32,
    fav_color: Color,
}

#[derive(Debug)]
enum Color {
    Red(String),
    Green,
    Blue,
}
impl Person {
    pub fn print(&self) -> String {
        format!(
            "name = {}, age={} has {} children",
            self.name, self.age, self.children
        )
    }
}
fn main() {
    let p = Person {
        name: "Bora".to_string(),
        age: 32,
        children: 3,
        fav_color: Color::Green,
    };

    let c = Color::Red("Bright".to_string());

    match c {
        Color::Red(s) => println!("Color is {} Red", s),
        Color::Blue => println!("Color is Blue"),
        Color::Green => println!("Color is Green"),
    };

    println!("Hello, people!, from {}", p.print());
    println!("Hello, people!, from {:?}", &p);
}
