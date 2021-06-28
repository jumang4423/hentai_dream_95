use std::io::*;
use std::str::from_utf8;

pub fn scan_me(print_data: &str) -> String {
    print!("{}", print_data);
    stdout().flush().unwrap();
    let mut _scaned_string = String::new();
    stdin()
        .read_line(&mut _scaned_string)
        .expect("input error => ?");

    _scaned_string =
        String::from(from_utf8(&_scaned_string.replace("\n", "").into_bytes()).unwrap());
    return _scaned_string;
}
