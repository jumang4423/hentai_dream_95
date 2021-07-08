use std::fs;

pub fn put_hentai(hentai_file: &String, dir: &String, password: &String) -> String {
    let joined: String = format!(
        "{}/{}/{}",
        dir,
        &password
            .chars()
            .map(|_temp| _temp.to_string())
            .collect::<Vec<String>>()
            .join("/"),
        hentai_file
    );
    match fs::copy(hentai_file, &joined) {
        Ok(_) => return joined,
        Err(_) => return "no file".to_string()
    }
}
