use std::{env, fs, io::{self, Error}};

pub fn read_from_args() -> io::Result<String> {
    let args: Vec<String> = env::args().collect();

    let path = args.get(1).ok_or_else(|| Error::new(io::ErrorKind::InvalidInput, "Missing file path."))?;
    fs::read_to_string(path)
}
