// #[allow(unused_variables)]
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

// enum Command {
//     Show,
//     New,
//     Edit,
//     Delete,
// }

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    println!("{:?}", args);
    // lets try to make a todo CLI tool.

    let todo_path = Path::new("./todo.txt");
    let mut todo_file = File::open(todo_path).unwrap();
    let mut todo_buf = String::new();
    todo_file.read_to_string(&mut todo_buf).unwrap();

    // Now parsing todo_buf
    // I have to break the todos into a Vec<String>
    let v: Vec<String> = todo_buf.split('\n').map(|line| line.to_string()).collect();
    print!("{:?}", v);
}
