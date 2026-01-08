use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

fn main(){
    let path = Path::new("./ssh.txt");
    
    if !path.exists() {
        fs::write("./ssh.txt", "").expect("blam");
    }

    let current_content = fs
        ::read_to_string(path)
        .expect("It should had woked in the first place.");

    println!("Current Content: {current_content}");
    println!("Enter something to put to file ssh.txt: ");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Booo...");

    // println!("You entered: {}", input);

    // Match practice, to understand it better.
    let some_var = 21;
    match some_var {
        32 => println!("I am 32"),
        21 => println!("--"),
        _ => println!("Wrong input"),
    }

    enum Test_enum_layout{
        ook(),
        NoOk(),
    };

    // put Test to 
    // no_ok
    let test_enum_in_memory = Test_enum_layout::NoOk();

    let output = match test_enum_in_memory{
        Test_enum_layout::ook() => {0}
        Test_enum_layout::NoOk() => {1},
    };
    println!("Match output is : {}", output);

    // Okay, Now I somewhat understand the match thing in rust.
    

    // Open the ssh.txt in Write only mode.
    
    let mut file = match File::create(&path) {
        Ok(file) => {
            file},
        Err(error) => {
            panic!("{error}")
        }
    };

    match file.write_all(&input.as_bytes()) {
        Err(why) => panic!("couldn't write to: {}", why),
        Ok(_) => println!("successfully wrote to ssh.txt",),
    }

}

