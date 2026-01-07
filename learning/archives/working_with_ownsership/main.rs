fn main(){
    let mut s = "Hello";
    println!("{s}");

    s = "world";
    println!("{s}");

    // Works, cause the string was on the stack.
    let mut ss = String::from("Baltej");
    ss.push_str(" works!");
    println!("{ss}");

    // So the ownership exists in rust, so if I write a variable whose ownership has been taken
    // will not work 
    let a = String::from("Wow this works");
    let b = a.clone(); // When you see a call to clone,
    // you know that some arbitrary code is being executed and that code may be expensive.
    // It’s a visual indicator that something different is going on.
    println!("b:{b} and a:{a}");
}

