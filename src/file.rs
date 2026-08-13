use core::error;
use std::fs::File;

fn main(){
    let file = File::create("hello.txt");

    // error handling -
    match File::create("hello.txt"){
        Ok(file)=>println!("File 'hello.txt' created successfully."),
        Err(error)=>rintln!("File 'hello.txt' not  created."),
    }
}