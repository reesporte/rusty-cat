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
            let handle = stdin.lock();
            for line in handle.lines() {
                println!("{}", &line.unwrap());
            }
        } else if dashu {
            let f = File::open(file)?;
            let reader = BufReader::new(f);
            for line in reader.lines() {
                println!("{}", &line.unwrap());
            }
        } else {
            let s = read_to_string(file)?;
            print!("{}", s);
        }
    }
    Ok(())
}
