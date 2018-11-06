pub fn test () {
    println!("{}", "Hello from Control");

    let flag = 100;
    if flag > 100 {
        println!("{}", "Greater than 100");
    }

    let name = if flag > 100 {"C++"} else {"Rust"};
    println!("{}", name);
}
