fn print_value(x: i32) {
    println!("value = {}", x);
}

// takes a mutable x of i32
fn increase(x: &mut i32) {
    *x += 1000;
}

// function with return type
fn product(x: i32, y: i32) -> i32 {
     x * y
}

fn functions() {
    print_value(999);

    let mut z = 999;
    increase(&mut z);

    println!("{}", z);

    let a = 3;
    let b = 10;

    let z = product(a, b);

    println!("{}", z);
}


fn main() {
    functions();
}
