// Quicky write all the code that you I have done before.

fn main() {
    println!("This is the comming ffrom a  file which was once golang file.");

    // Variable declaration.
    //
    let a = 21;
    let b: i32 = 21;

    let mut c = 21;
    c = a + b + c;

    println!("Sum or {} and {} and 21 is {}", a, b, c);

    // Defining an const variable;
    const NEOVIM: i32 = 1892 * 1421;
    println!("The value of the const is {}", NEOVIM);

    // Shadowing.
    //
    let d = 21;
    {
        let d = 43;
        println!("d = {}", d);
    }
    println!("d = {}", d);

    //  defining fuctions.
    //
    // Can we define function in main function itself?
    // If yes, It can mean we can define any function inside of any other fuction?

    fn some_function(a: i32, b: i32) -> i32 {
        a + b }
    // println!("{}", some_function(21, 21));

    if some_function(21, 21) == 42 {
        println!("The sum is indeed {}", 42);
    } else {
        println!("The sumn is not 42 and is {}", some_function(21, 21));
    }
}
