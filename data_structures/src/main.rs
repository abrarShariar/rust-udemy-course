#![allow(dead_code)]
#![allow(unused_variables)]

mod sh;
mod control;

union IntOrFloat {
    i: i32,
    f: f32
}

fn unions () {
    let mut iof = IntOrFloat { i:123 };
    iof.i = 999;

    let value = unsafe { iof.i };
}

struct Point {
    x: f64,
    y: f64
}

enum Color {
    Red,
    Green,
    Blue,
    RGBColor(u8,u8,u8)
}

struct Line {
    start: Point,
    end: Point
}


fn structures () {
    let mut p1 = Point { x: 10.99, y: 9.99 };
    let mut p2 = Point { x: 11.99, y: 10.20 };
    println!("{}", p1.x + p1.y);

    let myline = Line { start: p1, end: p2 };

    println!("{}", myline.start.x + myline.end.x);
    // p.x = 999.123;
    // println!("{}", p.x);
}

fn enums () {
    let c:Color = Color::Red;
    match c {
        Color::Red => println!("R"),
        Color::Green => println!("G"),
        Color::Blue => println!("B"),
        Color::RGBColor(0,0,0) => println!("Black"),
        _ => println!("YO")
    }
}


fn option() {
    // Option<T>
    let x = 3.0;
    let y = 0.0;

    let result:Option<f64> =
        if y != 0.0 { Some(x/y) } else { None };

    // println!("{:?}", result);
    match (result) {
        Some(z) => println!("result is {}", z),
        None => println!("None")
    }
}

fn use_slice (slice: &[i32]) {
    println!("first element = {}, length - {}", slice[0], slice.len());
}

fn slices () {
    let mut data = [1,2,3,4,5];
    use_slice(&data[0..4]);
}

fn main() {
    // sh::stack_and_heap();
    // control::test();
    // enums();
    // structures();
    // option();
    slices();
}
