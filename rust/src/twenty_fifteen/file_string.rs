use std::{path::Path, fs::File, io::Read};

pub fn file_string(file_path: String) -> String {
    let path = Path::new(&file_path);
    let display = path.display();

    // open file 
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // read the file into string 
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("Success file read"),
    };
    s
}
