use std::ops::Add;
use std::process::Command;
use std::env;

pub fn exec_program(bin_name: &str, args: &mut Vec<String>) -> () {
    let full_path_bin: Option<String> = find_full_binary(bin_name);
    if let Some(p) = full_path_bin {
        let output = Command::new(p)
            .args(args)
            .output()
            .expect("Can't execute the following program");

        println!("Result : {:?}", output);
    } else {
        println!("The following program doesn't exist.");
    }
}

fn find_full_binary(bin_name: &str) -> Option<String> {
    let path_env: String = env::var("PATH").expect("An error occured while accessing the PATH variable");
    let path_arr: Vec<&str> = path_env.split(";").collect();

    for p in path_arr.iter() {
        let full_path: String = format!("{p}\\{bin_name}");
        if std::path::Path::new(full_path.as_str()).exists() {
            return Some(full_path)
        }
    }

    None
}