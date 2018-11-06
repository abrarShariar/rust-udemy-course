#![allow(dead_code)]

use std::mem;

struct Point {
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point{x: 0.0, y: 0.0}
}

fn return_int() -> i32 {
    999
}

fn return_string() -> String {
    String::from("Hello World!")
}

pub fn stack_and_heap() {
    println!("{}",return_int());
    let x = return_int();
    println!("{}", x);

    let mut y = Box::new(return_int());

    println!("{} bytes", mem::size_of_val(&y));
    println!("{}", &y);

    let mut y2 = *y;
    println!("{}", y2);

    y2 = 100;

    println!("{}", y2);
    println!("{}", *y);

    *y = 0;

    println!("{}", y2);
    println!("{}", *y);

}
