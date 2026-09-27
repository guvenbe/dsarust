#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: i32,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
   pub fn new (x: i32, y: i32) -> Self{
        Point { x, y }
    } 
}
fn main() {
    let mut x = 34;
    let y = x;
    x += 5;
    println!("y = {}, x = {}", x ,y);


    let mut p = Person {
        name: "Bora".to_string(),
        age: 32,
    };

    let p2 = p.clone();
    p.name.push_str(" Brittney");
    println!("p={:?}, p2={:?}", p,p2);

    let mut pn1 = Point::new(3, 4);
    
    let pn2 = pn1;
    pn1.x += 4;
    println!("pn1= {:?}", pn1);
    println!("pn2= {:?}", pn2);
}
