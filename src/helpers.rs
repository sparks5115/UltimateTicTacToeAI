use std::fs::read_to_string;
use std::thread::sleep;
use std::time;

pub fn file_to_string(name: &str, wait_for_file: bool) -> String {
    loop{
        let file_string_result = read_to_string(name);
        match file_string_result {
            Ok(string) => return string,
            Err(error) => {
                if !wait_for_file {
                    panic!("File Not Found: {} \n Error: {}", name, error)
                }
            }
        };
        sleep(time::Duration::from_millis(10));
    }
}