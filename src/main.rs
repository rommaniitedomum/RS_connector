use communicator::network::server;
use communicator::network::client;
use communicator::network::math::{add, subtract};
// mod utilities {
//     pub fn print_hello() {
//         println!("Hello!");
//     }

//     pub fn print_goodbye() {
//         println!("Goodbye!");
//     }
// }
// use utilities::{print_hello, print_goodbye};


// mod utilities {
//     pub fn print_message(){
//         println!("better than JS");
//     }
// }

// use utilities::print_message as pm;


fn main() {
    server::start_server();
    client::connect_client();

    let sum = add(5,3);
    let difference = subtract(5, 3);

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);

//     enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }

// let row = vec![
//     SpreadsheetCell::Int(3),
//     SpreadsheetCell::Text(String::from("blue")),
//     SpreadsheetCell::Float(10.12),
// ];


// let mut s1 = String::from("foo");
// let s2 = "bar";
// s1.push_str(&s2);
// println!("s2 is {}", s2);

// let mut s = String::from("lo");
// s.push('l');

// println!("{}", s);

// let s1 = String::from("tic");
// let s2 = String::from("tac");
// let s3 = String::from("toe");

// let s = format!("{}-{}-{}", s1, s2, s3);

//     println!("{}",s);

//     for c in "안녕하세요".chars() {
//         println!("{}", c);
//     }

//     for b in "안녕하세요".bytes() {
//     println!("{}", b);
// }

    // pm();
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

if let Some(score) = scores.get("Blue") {
    println!("Blue team's score is {}", score);
}

if let Some(score) = scores.get("Yellow") {
    println!("Yellow team's score is {}", score);
}


scores.insert(String::from("Blue"), 20);

if let Some(score) = scores.get("Blue") {
    println!("New !!!!Blue team's score is {}", score);
}


scores.entry(String::from("Red")).or_insert(50);

//  전체 for문 반복 처리 출력 
for (key, value) in &scores {
    println!("{}: {}", key, value);
}



    // panic!("crash and burn");
    // Rustc 디버깅 하는법 = RUST_BACKTRACE=1 cargo run
    // 디버깅 1. 언랩 = 기본 메시지 
    // 2. expect : 에러 메시지 설정 

    use std::fs::File;


    let f = File::open("hello.txt").expect("Failed to open hello.txt");


}