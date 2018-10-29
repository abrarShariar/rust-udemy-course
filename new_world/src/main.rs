use std::mem;

fn main() {

    // variable a = unsigned 8 bits (0...255)
    let a:u8 = 255;
    println!("a = {}", a);

    // a = 456; //error for immutability

    let mut b:i8 = -100; // mutable
    println!("b = {}", b);
    b = 99;
    println!("b = {}", b);

    let mut c = 1234345; // 32 bit signed i32
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    // i8, u8, i16, u16, i64, u64

    c = -1;
    println!("c = {} after modification", c);

    let z:isize = 123; //isize/usize = size of pointer depends on computer architecture
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS",z, size_of_z, size_of_z * 8);

    //character
    let d:char = 'x';
    println!("d = {}, size of d = {} bytes", d, mem::size_of_val(&d));

    let e:f64 = 2.5; // double precision value (8 bytes)
    println!("e = {}, size of e = {} bytes", e, mem::size_of_val(&e));

    //BOOLEAN
    let g = false; //(1 bytes)
    println!("g = {}, size of g = {} bytes", g, mem::size_of_val(&g));

}
