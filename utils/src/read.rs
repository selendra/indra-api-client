use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_wasm(file_name: String) -> Vec<u8> {
    let path = Path::new(&file_name);
    if !path.exists() {
        eprintln!("Your file not exit");
        std::process::exit(2)
    };
    let mut file_content = Vec::new();
    let mut file = File::open(&file_name).expect("Unable to open file");
    file.read_to_end(&mut file_content).expect("Unable to read");
    file_content
}
