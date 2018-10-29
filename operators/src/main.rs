
fn main() {
    let mut a = 1+234+123*80;
    println!("{}",a);
    a += 1;
    a -= 2;
    println!("{}",a);
    println!("remainder = {}", (a%5));
    let a_cubed = i16::pow(a,3);
    println!("{}", a_cubed);
}
