use std::fs;

pub fn dft_folder_maker(array: Vec<u8>, count_mut: usize, dir: &String) {
    if count_mut != 0 {
        for i in 0..10 {
            let mut new_array = array.clone();
            new_array.push(i as u8);
            let joined: String = String::from(dir.clone() + &String::from("/"))
                + &new_array
                    .clone()
                    .iter()
                    .map(|_temp| _temp.to_string())
                    .collect::<Vec<String>>()
                    .join("/");

            fs::create_dir_all(joined).unwrap();
            dft_folder_maker(new_array, count_mut - 1, &dir);
        }
    }
}