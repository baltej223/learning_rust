use std::env;
use std::fs;
use std::io;
fn main() {
    let CurrentWorkingDirectory = env::current_dir().expect("I dont want to handle errors.");
    let currennt_dir = fs::read_dir(&CurrentWorkingDirectory).expect("Some error occured.");

    println!("Files in {:#?} are:", &CurrentWorkingDirectory);

    let mut dir_vec: Vec<std::path::PathBuf> = Vec::new();

    for item in currennt_dir {
        let entry = item.expect("Failed to get directory entry");
        let path_of_item = entry.path();
        &dir_vec.push(path_of_item);
    }
    loop {
        let mut i = 0;
        for item in &dir_vec {
            println!("{}:{}", i + 1, dir_vec[i].display());
            i = i + 1;
        }
        OpenFile(&dir_vec);
    }
}

fn OpenFile(v: &Vec<std::path::PathBuf>) {
    println!("Which file you want to open, Enter number: ");
    let mut file_number = String::new();

    io::stdin()
        .read_line(&mut file_number)
        .expect("Some error occured.");

    let file_number_: usize = file_number.trim().parse().expect("Failed To parse");

    let the_file_to_read = &v[file_number_ - 1];

    let file_data = fs::read_to_string(the_file_to_read).expect("Please don't fail");
    println!("Content:\n{file_data}");
}
