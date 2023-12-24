use std::{fs::File, io::Read};

pub fn readfile_by_lines(file_name: &str) -> Vec<String> {
    let mut file = File::open(file_name).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    contents
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}
