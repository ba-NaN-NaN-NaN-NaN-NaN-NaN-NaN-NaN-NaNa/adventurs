use std::fs::File;
use std::io::prelude::*;
use std::env;

#[allow(dead_code)]
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

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(fpath) {
        Err(why) => return Err(why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => return Err(why),
        Ok(_) => {
            // print!("{} contains:\n{}", fpath.display(), s)
        }
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

#[allow(dead_code)]
pub fn rows_to_ints(input: &str) -> Vec<i64> {
    let lines = input.split("\n");
    let mut toreturn = Vec::new();
    for line in lines {
        let trimmed = line.trim();
        if trimmed.len() == 0 {
            continue;
        }

        toreturn.push(trimmed.parse::<i64>().unwrap());
    }
    if toreturn.len() == 0 {
        panic!("Something wrong, rows_to_ints has nothing to return. Only whitespace???");
    }
    return toreturn
}
