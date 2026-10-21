fn main(){
    for i in 0..10{
        println!("naive = {}, iter = {}, dynamic = {}, tail = {}", fibonacci(i), fibonacci_iter(i), fibonacci_dynamic(i).0, fibonacci_tail(i));
    }
}

pub fn fibonacci(n: i32) -> i32{
    if n<=1 {
        return 1;
    }
    fibonacci(n-1) + fibonacci(n-2)
}

pub fn fibonacci_iter(n: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;
    let mut res = 1;

    for _ in 1..n {
        res = a + b;
        a=b;
        b= res;
    }
    res
}

pub fn fibonacci_dynamic(n: i32)->(i32, i32){
    if n == 0 {
        return (1, 0);
    }
    let (a, b) = fibonacci_dynamic(n-1);
    (a + b, a)
}

pub fn fibonacci_tail(n: i32) -> i32 {
    fibonacci_tail_helper(n, 1, 1)
}

fn fibonacci_tail_helper(n: i32, a: i32, b: i32) -> i32 {
    if n <= 1 {
        return b;
    }
    fibonacci_tail_helper(n-1, b, a + b)
}
