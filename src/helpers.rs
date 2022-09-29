use std::thread::sleep;
use std::{fs, time};

pub fn file_to_string(name: &str, wait_for_file: bool) -> String {

    let file_string_result = fs::read_to_string(name);
    let file_string = match file_string_result {
        Ok(string) => string,
        Err(error) => {
            if !wait_for_file {
                panic!("File Not Found: {} \n Error: {}", name, error)
            }else{
                sleep(time::Duration::from_millis(10));
                return file_to_string(name, wait_for_file);
            }
        }
    };
    return file_string;
}