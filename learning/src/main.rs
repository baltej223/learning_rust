fn main() {
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

    // # Refrences and borrowing.
    let mut c = String::from("Nightly");
    baltejs_refrence(&c);
    // baltejs_refrence(&c);
    let ref1 = &mut c;
    let ref2 = &mut c; // here ref1 just ended.
                       // println!("{ref1}, {ref2}");

    // let ref3 = &c;
    // let ref4 = &mut c;
    // // print!("{ref3}  and  {ref4}\n");
    // // Cant use a mutable and an imutable refrence together.

    // Slice type work
    println!(
        "The first word is : {}",
        WhichReturnsTheFirstWordOutOfAString(String::from("Hey I am baltej"))
    );
}

fn baltejs_refrence(s: &String) {
    let len = s.len();
    println!("The passed string's length is : {len}");
}

// fn letsCreateADanglingPointer()->&String { // Lifetime parameter error.
//     let string = String::from("This is the string");
//     &string
// }
//
fn no_dangle() -> String {
    String::new()
}

fn WhichReturnsTheFirstWordOutOfAString(s: String) -> String {
    let mut res = String::new();
    for charr in s.chars() {
        if charr == ' ' {
            return res;
        }
        res.push(charr);
    }
    res
}
