use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

pub fn get_input(fname: &str) -> Result<std::path::PathBuf, std::io::Error> {
    // Read an input file.
    // All path resolution will be done here.
    let mut path = match env::current_dir() {
        Err(e) => {
            println!("Fatal: Current directory does not exist?");
            return Err(e)
        }
        Ok(p) => p
    };

    path = match path.canonicalize() {
        Err(e) => {
            println!("Fatal: Current directory can not be resolved?");
            return Err(e)
        }
        Ok(p) => p
    };


    path.push("input");
    path.push(fname);
    path.canonicalize()
}

pub fn readstring(fpath: &std::path::PathBuf) -> Result<String, std::io::Error> {
    // Read entire file into string.

    // Create a path to the desired file    
    let display = fpath.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(fpath) {
        Err(why) => return Err(why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => return Err(why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    return Ok(s)
}

#[allow(dead_code)]
pub fn readlines(fname: &str) -> Result<Vec<String>, std::io::Error> {
    // Read entire file into list of strings.
    match readstring(&std::path::PathBuf::from(fname)) {
        Err(why) => return Err(why),
        Ok(s) => {
            let mut res = Vec::<String>::new();
            for part in s.split("\n") {
                res.push(String::from(part))
            }
            Ok(res)
        }
    }
}

