use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let result: Result<String, io::Error> = read_username_from_file();

    match result {
        Ok(name) => println!("{}", name),
        Err(e) => println!("error -> {:?}", e)
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    /*
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };
    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => return Err(e)
    }
    */

    let mut s: String = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}