use std::io::*;
use std::fs;

fn main() {
    
        // PASSWORD
        print!(
            "PASSWORD:"
        );
        stdout().flush().unwrap();
        let mut password = String::new();
        stdin().read_line(&mut password).expect("input error => ?");

        // DIRECTORY
        print!(
            "HENTAI FILE:"
        );
        stdout().flush().unwrap();
        let mut hentai = String::new();
        stdin().read_line(&mut hentai).expect("input error => ?");

        // FOLDER NAME
        print!(
            "HENTAI DIRECTORY NAME:"
        );
        stdout().flush().unwrap();
        let mut dir = String::new();
        stdin().read_line(&mut dir).expect("input error => ?");

        // LETSGO

        print!(
            "HENTAI DREAM..."
        );

        hentai_dream(password, hentai, dir);
}

fn hentai_dream(password: String, hentai: String,dir: String) {
    fs::create_dir_all(dir).unwrap();
}