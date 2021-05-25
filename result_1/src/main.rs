use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file_name: String = String::from("hello.txt");

    /*
    let f = File::open(&file_name);
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create(&file_name) {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}", e);
                }
            }
        }
        Err(error) => {
            panic!("There was a problem opening the file {:?}", error);
        }
    };

    println!("File result: {:#?}", f);
    */

    // shortcut, unwrap and expect
    let f = File::open(&file_name).unwrap(); // 파일이 존재하지 않을 경우 panic! macro 호출
}