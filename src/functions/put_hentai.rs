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
    fs::copy(hentai_file, &joined).unwrap();

    joined
}