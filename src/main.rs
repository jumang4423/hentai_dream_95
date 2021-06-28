use std::fs;

mod functions;

fn main() {

    let password = functions::scan_me::scan_me("? enter password (length<=6):");
    let hentai = functions::scan_me::scan_me("? choose your hentai file:");
    let dir = functions::scan_me::scan_me("? enter hentai directory name:");

    println!("> hentai dream processing");
    print!("> ");

    // make tons of directories (lol)
    fs::create_dir_all(&dir).unwrap();
    functions::dft_folder_maker::dft_folder_maker([].to_vec(), password.chars().count(), &dir);

    println!("");
    // copy hentai file
    println!(
        "! {} folders were created.",
        i32::pow(10, password.chars().count() as u32)
    );
    println!("> your dream at:");
    println!("> {}", functions::put_hentai::put_hentai(&hentai, &dir, &password));
    println!("> enjoy hentai bruh :8)");
}