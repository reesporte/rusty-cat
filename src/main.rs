use std::env::args;
use std::fs::{read_to_string, File};
use std::io::{stdin, BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();

    let mut dashu = false; // uncle!
    let mut files = Vec::<String>::new();

    if args.len() > 1 {
        let mut i = 1;
        if args[1] == "-u" {
            dashu = true;
            i = 2;
        }
        while i < args.len() {
            files.push(args[i].clone());
            i += 1;
        }
    }

    if files.len() == 0 {
        files.push("-".to_string());
    }

    for file in files {
        if file == "-" {
            let stdin = stdin();
            for line in stdin.lock().lines() {
                println!("{}", &line.unwrap());
            }
        } else if dashu {
            for line in BufReader::new(File::open(file)?).lines() {
                println!("{}", &line.unwrap());
            }
        } else {
            print!("{}", read_to_string(file)?);
        }
    }
    Ok(())
}
