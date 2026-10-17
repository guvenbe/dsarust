use std::fmt::format;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct LinkedList<T> {
    data: T,
    next: Option<Box<LinkedList<T>>>,
}

impl<T: std::ops::AddAssign> LinkedList<T> {
    pub fn add_up(&mut self, n: T) {
        self.data += n;
    }
}
fn main() {
    let mut ll = LinkedList {
        data: 3,
        next: Some(Box::new(LinkedList {
            data: 2,
            next: None,
        })),
    };
    if let Some(ref mut v) = ll.next {
        v.add_up(10);
    }
    println!("ll = {:?}", ll);

    let mut v: Vec<String> = Vec::with_capacity(100);
    v.push("hello".to_string());
    v.push("goodbye".to_string());

    for i in 0..105 {
        v.push(i.to_string());
    }

    println!("v.len = {}, capacity= {}", v.len(), v.capacity());
    let mut s = "hello".to_string();

    let ferris = "🦀🦀🦀🦀"; // char literal
    println!("{ferris}");
    let ferris = '\u{1F980}'; // escape form — same char
    println!("{ferris}");
    let s = "こんにちは";
    println!("{s}");
    let s1 = s.to_string();
    let s2 = String::from(s);
    let s3 = s.to_owned();
    let s4 = &s1[..];
    let mut s5 = String::from("foo");
    s5.push_str("bar");
    println!("{}", s5);
    s5.replace_range(..,"baz");
    let s7 = "!".to_string();
    let s6 = s5 + &s7;
    println!("{}",s6);
    let s8 = String::from("tic");
    let s9 = String::from("tac");
    let s10 = String::from("toe");
    let s11 =  format!("{}-{}-{}", s8,s9,s10);
    let s13 = ["first","second"].concat();
    let s14 = concat!("fist", "secon");
    let s15 =s13 + "Third";
    let ferris2 = "🦀🦀🦀🦀"; // char literal
    let s16 = &ferris2[0..4];
    println!("s16={}", s16);
    for b in s.bytes(){
        println!("{}", b);
    }
    for b in s.chars(){
        println!("{}", b);
    }
    for g in ferris2.graphemes(true){
        println!("{}", g);
    }
    let s17 = "hello World!";
    let s18 = String::from("hello worls");
    my_function(s17);
    my_function(&s18);
    let fstr = "help me find home";
    let ffstr = string_find_f(fstr);
    println!("ffstr={}", ffstr);
    let choose =choose_str(1);
    println!("{}", choose);
}

fn my_function(a: &str) -> String {
    format!("{}", a)
}

fn string_find_f(s: &str) -> &str{
    for (n, x) in s.char_indices(){
        if x == 'f' {
            return &s[n..]; 
        }
    }
    s
}

fn choose_str(n: i32)->&'static str{
    match n {
        0 => "hello",
        1 => "goodbye",
        _ => "other",
        
    }
} 
