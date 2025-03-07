use std::io::{self, BufReader, Read, IsTerminal};

pub fn read_input(default_file: &str) -> Result<std::vec::IntoIter<char>, String> {
    let chars: std::vec::IntoIter<char> = if io::stdin().is_terminal() {
        let file = std::fs::File::open(default_file)
            .map_err(|err| format!("Error reading input: {}", err))?;
        
        let bytes: Vec<u8> = BufReader::new(file)
            .bytes()
            .filter_map(|b| b.ok())
            .collect();
            
        String::from_utf8_lossy(&bytes).chars().collect::<Vec<char>>().into_iter()
    } else {
        let bytes: Vec<u8> = io::stdin()
            .bytes()
            .filter_map(|b| b.ok())
            .collect();
            
        String::from_utf8_lossy(&bytes).chars().collect::<Vec<char>>().into_iter()
    };

    Ok(chars)
} 