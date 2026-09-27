#[derive(Debug)]
pub enum Res<T, E> {
    Thing(T),
    Error(E),
}
fn main() {
    println!("Hello, world!");
    let a = divide(4, 5);
    let b = divide(10, 0);
    println!("a={:?}, b={:?}", a, b);

    let c = divide(10, 5);
    let d = divide(10, 2);

    match c {
        Res::Thing(v) => println!("val = {}", v),
        _ => {}
    }

    if let Res::Thing(v) = d{

        println!("vald={}", v);
    }
    let e = divide2(10, 5);
    if let Ok(v) = e{
        println!("val = {}", v);
    }
}
fn divide(a: i32, b: i32) -> Res<i32, String>{
    if b == 0 {
        return Res::Error("Cannot divide by zero".to_string())
    }
    Res::Thing(a/b)
}

fn divide2(a: i32, b: i32) -> Result<i32, String>{
    if b == 0 {
        return Result::Err("Can't div by zero".to_string());
    }
    Ok(a/b)
}
