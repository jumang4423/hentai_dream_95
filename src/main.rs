use std::fs;
use std::io::*;
use std::str::from_utf8;

fn main() {
    // password
    let password = scan_me("? enter password (length<=6):");
    // file
    let hentai = scan_me("? choose your hentai file:");
    // folder name
    let dir = scan_me("? enter hentai directory name:");

    println!("> hentai dream processing\n> might take a few moment...\n");

    // make directories (lol)
    fs::create_dir_all(&dir).unwrap();
    dft_folder_maker([].to_vec(), password.chars().count(), &dir);

    // put hentai file
    println!("! {} folders were created.", i32::pow(10, password.chars().count() as u32));
    println!("> your dream at:");
    println!("> {}", put_hentai(&hentai, &dir, &password));
    println!("> enjoy hentai bruh");
}

fn scan_me(print_data: &str) -> String {
    print!("{}", print_data);
    stdout().flush().unwrap();
    let mut _scaned_string = String::new();
    stdin()
        .read_line(&mut _scaned_string)
        .expect("input error => ?");

    _scaned_string = String::from(from_utf8(&_scaned_string.replace("\n", "").into_bytes()).unwrap());
    return _scaned_string;
}

fn dft_folder_maker(array: Vec<i32>, count: usize, dir: &String) {
    if count != 0 {
        for i in 0..10 {
            let mut new_array = array.clone();
            new_array.push(i);
            let joined: String = String::from(dir.clone() + &String::from("/"))
                + &new_array
                    .clone()
                    .iter()
                    .map(|_temp| _temp.to_string())
                    .collect::<Vec<String>>()
                    .join("/");

            fs::create_dir_all(joined).unwrap();
            dft_folder_maker(new_array, count - 1, &dir);
        }
    }
}

fn put_hentai(hentai_file: &String, dir: &String, password: &String) -> String {

    let joined: String = format!("{}/{}/{}", dir, &password
    .chars()
    .map(|_temp| _temp.to_string())
    .collect::<Vec<String>>()
    .join("/"),hentai_file);
    fs::copy(hentai_file, &joined).unwrap();

    joined
}