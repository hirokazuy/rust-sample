use std::fs::File;
use std::io::ErrorKind;

fn test1() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        }
    };
}

fn test2() {
    let f = File::open("hello.txt").unwrap();
}

fn test3() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn main() {
//    test1();
//    test2();
    test3();
}
