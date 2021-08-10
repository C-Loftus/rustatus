use std::fs::OpenOptions;


pub fn setup_logger() -> std::fs::File {
    OpenOptions::new()
        .truncate(true)
        .read(true)
        .create(true)
        .write(true)
        .open(dirs::home_dir().unwrap().join("rustatus"))
        .unwrap()
}