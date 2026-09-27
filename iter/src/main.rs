pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.curr >= self.max {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}
fn main() {
    let mut n = 0;
    loop {
        n += 1;
        if n == 10 {
            break;
        }
        println!("Hello world {}", n);
    }
    println!("All Loop Done!");

    let mut n = 0;

    while n < 10 {
        n += 1;
        println!("Hello world {}", n);
    }
    println!("All done while!");

    for i in 1..10 {
        println!("for i = {}", i);
    }
    let mut st = Stepper {
        curr: 2,
        step: 3,
        max: 15,
    };

    loop {
        match st.next() {
            Some(v) => println!("loop {}", v),
            None => break,
        }
    }
    let mut st = Stepper {
        curr: 3,
        step: 4,
        max: 20,
    };
    while let Some(n) = st.next() {
        println!(" while {}", n);
    }
    let st = Stepper {
        curr: 5,
        step: 10,
        max: 60,
    };
    for i in st {
        println!("for loop {}", i);
    }
}
