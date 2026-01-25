use std::env::current_dir as cwd;
use std::fs;
use std::path::Path;

struct FilesInFolder {
    is_dir: bool,
    pathIs: std::path::PathBuf,
    file_name: std::ffi::OsString,
}

fn read_folder(reative_path: &std::path::Path) -> Vec<FilesInFolder> {
    let mut file_vec: Vec<FilesInFolder> = Vec::new();
    let dir_con = fs::read_dir(&reative_path).expect("1 Here.");

    for i in dir_con {
        let cur_fl = i.expect("2 Here");
        let file_type = cur_fl.file_type().expect("Failed to get file type");
        let is_dir = file_type.is_dir();
        let file_name = cur_fl.file_name();

        let file_struct = FilesInFolder {
            is_dir,
            pathIs: cur_fl.path(),
            file_name,
        };
        // Now pushing this to the vector
        let _ = &file_vec.push(file_struct);
    }
    file_vec
}

fn main() {
    // Get the current working directory
    let binding = cwd().expect("Here");
    let curr_dir = Path::new(&binding);

    recursive_print_insides_of_folder(curr_dir.to_path_buf(), 0);
}

fn n_tab_chars(i: i32) -> String {
    let mut to_ret = String::new();
    for _ in 1..=i {
        to_ret.push_str("    ");
    }
    to_ret
}

fn recursive_print_insides_of_folder(path_to: std::path::PathBuf, depth: i32) {
    // println!("Path which is being sent to the func is: {}", path_to.display());
    let this_folder: Vec<FilesInFolder> = read_folder(&path_to);

    for file in this_folder {
        if !file.is_dir {
            print!("{}", n_tab_chars(depth));
            println!("{}", file.file_name.display());
            continue;
        }
        print!("{}", n_tab_chars(depth));
        println!("{}", file.file_name.display());
        recursive_print_insides_of_folder(file.pathIs, depth + 1);
    }
}
