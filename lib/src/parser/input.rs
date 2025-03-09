use std::io::{BufReader, Read};

pub fn read_input(default_file: &str) -> Result<std::vec::IntoIter<char>, String> {
    let file = std::fs::File::open(default_file)
        .map_err(|err| format!("Error reading input: {}", err))?;
    
    let bytes: Vec<u8> = BufReader::new(file)
        .bytes()
        .filter_map(|b| b.ok())
        .collect();
                
    Ok(String::from_utf8_lossy(&bytes).chars().collect::<Vec<char>>().into_iter())
} 